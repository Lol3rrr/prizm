use sh::asm::{self, Instruction};

use crate::{
    general, instructiontype::InstructionType, system, traits::Debugger, Display, Exception, Input,
    Memory,
};

/// Emulates the CPU of the Calculator
pub struct CPU {
    pc: u32,
    /// A queued up Instruction that should be executed next
    /// (Address of Instruction, Instruction)
    queued_instr: Option<(u32, Instruction)>,
}

impl CPU {
    /// Creates a new empty CPU with the given Address as the initial PC value
    pub fn new(starting_point: u32) -> Self {
        Self {
            pc: starting_point,
            queued_instr: None,
        }
    }

    /// Returns the current PC
    pub fn pc(&self) -> u32 {
        self.pc
    }

    fn fetch_instruction(pc: u32, memory: &mut Memory) -> asm::Instruction {
        let word_bytes = memory.read_word(pc);
        asm::Instruction::parse(word_bytes)
    }
    fn fetch_instruction_type(pc: u32, memory: &mut Memory) -> InstructionType {
        InstructionType::parse(&Self::fetch_instruction(pc, memory))
    }

    /// Handles Jump instructions accordingly and queues up the next instruction
    /// in case of a delayed branch
    fn handle_jump(&mut self, memory: &mut Memory, destination: u32, delayed: bool) {
        if delayed {
            let tmp = Self::fetch_instruction(self.pc + 2, memory);
            self.queued_instr = Some((self.pc + 2, tmp));
        }

        self.pc = destination;
    }

    /// Executes the single given Instruction
    fn execute<D>(
        &mut self,
        instr: Instruction,
        memory: &mut Memory,
        display: &mut D,
        debugger: &dyn Debugger,
    ) -> Result<(), Exception>
    where
        D: Display,
    {
        match &instr {
            // Move Instructions
            Instruction::Mov(n_register, m_register) => {
                memory.write_register(*n_register, memory.read_register(*m_register));
                self.pc += 2;
            }
            Instruction::MovI(register, raw_value) => {
                let value = general::sign_extend_u8(*raw_value);
                memory.write_register(*register, value);
                self.pc += 2;
            }
            Instruction::MovA(raw_disp) => {
                let disp = ((*raw_disp as u32) & 0x000000FF) * 4;
                let pc = self.pc & 0xFFFFFFFC;
                let addr = disp + pc + 4;

                memory.write_register(0, addr);

                self.pc += 2;
            }
            Instruction::MovT(n_register) => {
                let value = if memory.t { 1 } else { 0 };
                memory.write_register(*n_register, value);

                self.pc += 2;
            }
            Instruction::MovB(target, source) => {
                let value = match source {
                    asm::Operand::AtRegister(m_register) => {
                        let raw_value = memory.read_byte(memory.read_register(*m_register));
                        general::sign_extend_u8(raw_value)
                    }
                    asm::Operand::Register(m_register) => {
                        memory.read_register(*m_register) & 0x000000FF
                    }
                    _ => unimplemented!("Unknown Source: {:?}", source),
                };

                match target {
                    asm::Operand::Register(n_register) => {
                        memory.write_register(*n_register, value);
                    }
                    asm::Operand::AtRegister(n_register) => {
                        let target_addr = memory.read_register(*n_register);
                        memory.write_byte(target_addr, value as u8, display);
                    }
                    _ => unimplemented!("Unknown Target: {:?}", target),
                };
                self.pc += 2;
            }
            Instruction::MovW(target, source) => {
                let value = match source {
                    asm::Operand::Register(m_register) => memory.read_register(*m_register),
                    asm::Operand::Displacement8(raw_disp) => {
                        let raw_immediate: u32 = 0x000000FF & (*raw_disp as u32);
                        let addr = self.pc + 4 + (raw_immediate * 2);
                        memory.read_word(addr) as u32
                    }
                    asm::Operand::OffsetR0(offset_reg) => {
                        let addr = memory.read_register(0) + memory.read_register(*offset_reg);
                        general::sign_extend_u16(memory.read_word(addr))
                    }
                    _ => unimplemented!("Unknown Source: {:?}", source),
                };

                match target {
                    asm::Operand::AtRegister(n_register) => {
                        let target_addr = memory.read_register(*n_register);
                        memory.write_word(target_addr, value as u16, display);
                    }
                    asm::Operand::Register(n_register) => {
                        memory.write_register(*n_register, value);
                    }
                    _ => unimplemented!("Unknown Target: {:?}", target),
                };
                self.pc += 2;
            }
            Instruction::MovL(target, source) => {
                let value = match source {
                    asm::Operand::Register(m_register) => memory.read_register(*m_register),
                    asm::Operand::AtRegister(m_register) => {
                        memory.read_long(memory.read_register(*m_register))
                    }
                    asm::Operand::Displacement8(raw_disp) => {
                        let raw_immediate: u32 = 0x000000FF & (*raw_disp) as u32;
                        let addr = (self.pc & 0xFFFFFFFC) + 4 + (raw_immediate * 4);
                        memory.read_long(addr)
                    }
                    asm::Operand::Displacement4Reg(disp, other) => {
                        let extended = 0x0000000F & (*disp as u32);
                        let addr = memory.read_register(*other) + extended * 4;
                        memory.read_long(addr)
                    }
                    _ => unimplemented!("Unknown Source: {:?}", source),
                };

                match target {
                    asm::Operand::Register(n_register) => {
                        memory.write_register(*n_register, value);
                    }
                    asm::Operand::AtRegister(n_register) => {
                        let addr = memory.read_register(*n_register);
                        memory.write_long(addr, value, display);
                    }
                    asm::Operand::OffsetR0(offset_reg) => {
                        let addr = memory.read_register(0) + memory.read_register(*offset_reg);
                        memory.write_long(addr, value, display);
                    }
                    asm::Operand::Displacement4Reg(disp, n_register) => {
                        let extended = 0x0000000F & (*disp as u32);
                        let addr = memory.read_register(*n_register) + extended * 4;
                        memory.write_long(addr, value, display);
                    }
                    _ => unimplemented!("Unknown Target: {:?}", target),
                };

                self.pc += 2;
            }
            Instruction::PopOther(n_register, m_register) => {
                let address = memory.read_register(*m_register);
                let value = memory.read_long(address);
                memory.write_register(*n_register, value);
                memory.write_register(*m_register, address + 4);
                self.pc += 2;
            }
            Instruction::PushOtherB(m_register, n_register) => {
                let mut n = memory.read_register(*n_register);
                n -= 1;
                memory.write_register(*n_register, n);
                let value = memory.read_register(*m_register);
                memory.write_byte(n, value as u8, display);

                self.pc += 2;
            }
            Instruction::PushOther(m_register, n_register) => {
                let mut n = memory.read_register(*n_register);
                n -= 4;
                memory.write_register(*n_register, n);
                memory.write_long(n, memory.read_register(*m_register), display);
                self.pc += 2;
            }
            Instruction::ExtuW(n_register, m_register) => {
                let prev_value = memory.read_register(*m_register);
                let extended_value = 0x0000FFFF & prev_value;
                memory.write_register(*n_register, extended_value);

                self.pc += 2;
            }

            // Shift instructions
            Instruction::Shar(n_register) => {
                memory.t = (memory.read_register(*n_register) & 0x00000001) != 0;

                let prev_value = memory.read_register(*n_register);
                let n_value = if (prev_value & 0x80000000) != 0 {
                    (prev_value >> 1) | 0x80000000
                } else {
                    (prev_value >> 1) & 0x7FFFFFFF
                };
                memory.write_register(*n_register, n_value);

                self.pc += 2;
            }
            Instruction::Shll(n_register) => {
                let value = memory.read_register(*n_register);
                memory.t = (value & 0x80000000) != 0;

                memory.write_register(*n_register, value << 1);
                self.pc += 2;
            }
            Instruction::Shll2(register) => {
                let data = memory.read_register(*register);
                memory.write_register(*register, data << 2);
                self.pc += 2;
            }
            Instruction::Shll8(register) => {
                let data = memory.read_register(*register);
                memory.write_register(*register, data << 8);
                self.pc += 2;
            }
            Instruction::Shll16(register) => {
                let data = memory.read_register(*register);
                memory.write_register(*register, data << 16);
                self.pc += 2;
            }
            Instruction::Shld(n_register, m_register) => {
                let raw_shift = memory.read_register(*m_register);
                let shift_value: u8 = (raw_shift as u8) & 0x1f;

                let prev_value = memory.read_register(*n_register);
                let new_value = if (raw_shift & 0x80000000) == 0 {
                    prev_value << shift_value
                } else {
                    unimplemented!("[Shld] Shift-Right");
                };

                memory.write_register(*n_register, new_value);

                self.pc += 2;
            }
            Instruction::Shlr(n_register) => {
                let value = memory.read_register(*n_register);
                memory.t = (value & 0x00000001) != 0;

                memory.write_register(*n_register, value >> 1);
                self.pc += 2;
            }
            Instruction::Shlr2(register) => {
                let data = memory.read_register(*register);
                memory.write_register(*register, data >> 2);
                self.pc += 2;
            }
            Instruction::Shlr8(register) => {
                let data = memory.read_register(*register);
                memory.write_register(*register, data >> 8);
                self.pc += 2;
            }
            Instruction::Shlr16(register) => {
                let data = memory.read_register(*register);
                memory.write_register(*register, data >> 16);
                self.pc += 2;
            }

            // Arithmetic
            Instruction::Sub(target, other) => {
                let new_value = memory
                    .read_register(*target)
                    .wrapping_sub(memory.read_register(*other));
                memory.write_register(*target, new_value);

                self.pc += 2;
            }
            Instruction::Subc(target, other) => {
                let target_value = memory.read_register(*target);
                let other_value = memory.read_register(*other);
                let t_value = if memory.t { 0 } else { 1 };

                let tmp1 = target_value.wrapping_sub(other_value);
                let tmp0 = target_value;
                let n_value = tmp1.wrapping_sub(t_value);
                memory.write_register(*target, n_value);

                memory.t = tmp0 < tmp1;
                if tmp1 < n_value {
                    memory.t = true;
                }

                self.pc += 2;
            }
            Instruction::Add(target, other) => {
                let target_value = memory.read_register(*target);
                let other_value = memory.read_register(*other);
                let final_value = target_value.wrapping_add(other_value);

                memory.write_register(*target, final_value);
                self.pc += 2;
            }
            Instruction::AddI(register, value) => {
                let add_value = general::sign_extend_u8(*value);

                memory.write_register(
                    *register,
                    memory.read_register(*register).wrapping_add(add_value),
                );
                self.pc += 2
            }
            Instruction::MulL(first, second) => {
                let first_data = memory.read_register(*first);
                let second_data = memory.read_register(*second);

                memory.macl = first_data.wrapping_mul(second_data);

                self.pc += 2;
            }
            Instruction::DmulSL(first, second) => {
                let first_data = memory.read_register(*first) as i64;
                let second_data = memory.read_register(*second) as i64;

                let result = first_data * second_data;

                let n_mach = (result >> 32) as u32;
                let n_macl = result as u32;

                memory.mach = n_mach;
                memory.macl = n_macl;

                self.pc += 2;
            }

            // Branch Instructions
            Instruction::Jmp(register) => {
                match Self::fetch_instruction_type(self.pc + 2, memory) {
                    InstructionType::Branch => return Err(Exception::SlotIllegal),
                    _ => {}
                };

                let destination = memory.read_register(*register);

                self.handle_jump(memory, destination, true);
            }
            Instruction::BT(raw_disp) => {
                let disp = general::sign_extend_u8(*raw_disp) << 1;
                if memory.t {
                    self.handle_jump(memory, self.pc + 4 + disp, false);
                } else {
                    self.pc += 2;
                }
            }
            Instruction::BTs(raw_disp) => {
                let disp = general::sign_extend_u8(*raw_disp) << 1;
                if memory.t {
                    self.handle_jump(memory, self.pc + 4 + disp, true);
                } else {
                    self.pc += 2;
                }
            }
            Instruction::BF(raw_disp) => {
                let disp = general::sign_extend_u8(*raw_disp) << 1;
                if !memory.t {
                    self.handle_jump(memory, self.pc + 4 + disp, false);
                } else {
                    self.pc += 2;
                }
            }
            Instruction::BFs(raw_disp) => {
                let disp = general::sign_extend_u8(*raw_disp) << 1;
                if !memory.t {
                    let target = self.pc.wrapping_add(disp).wrapping_add(4);
                    self.handle_jump(memory, target, true);
                } else {
                    self.pc += 2;
                }
            }
            Instruction::Jsr(m_register) => {
                match Self::fetch_instruction_type(self.pc + 2, memory) {
                    InstructionType::Branch => return Err(Exception::SlotIllegal),
                    _ => {}
                };

                let destination = memory.read_register(*m_register);
                memory.pr = self.pc + 4;
                self.handle_jump(memory, destination, true);
            }
            Instruction::Rts => {
                match Self::fetch_instruction_type(self.pc + 2, memory) {
                    InstructionType::Branch => return Err(Exception::SlotIllegal),
                    _ => {}
                };

                let destination = memory.pr;
                self.handle_jump(memory, destination, true);
            }
            Instruction::BRA(raw_disp) => {
                let disp = general::sign_extend_u12(*raw_disp) << 1;

                match Self::fetch_instruction_type(self.pc + 2, memory) {
                    InstructionType::Branch => return Err(Exception::SlotIllegal),
                    _ => {}
                };

                let target = self.pc.wrapping_add(4).wrapping_add(disp);
                self.handle_jump(memory, target, true);
            }
            Instruction::BSR(raw_disp) => {
                let disp = general::sign_extend_u12(*raw_disp) << 1;

                match Self::fetch_instruction_type(self.pc + 2, memory) {
                    InstructionType::Branch => return Err(Exception::SlotIllegal),
                    _ => {}
                };

                memory.pr = self.pc + 4;
                let target = self.pc.wrapping_add(4).wrapping_add(disp);
                self.handle_jump(memory, target, true);
            }

            // Comparisons
            Instruction::CmpEqI(raw_im) => {
                let immediate = general::sign_extend_u8(*raw_im);
                memory.t = memory.read_register(0) == immediate as u32;

                self.pc += 2;
            }
            Instruction::CmpEq(n_register, m_register) => {
                memory.t = memory.read_register(*n_register) == memory.read_register(*m_register);

                self.pc += 2;
            }
            Instruction::CmpHs(n_register, m_register) => {
                memory.t = memory.read_register(*n_register) >= memory.read_register(*m_register);

                self.pc += 2;
            }
            Instruction::CmpHi(n_register, m_register) => {
                memory.t = memory.read_register(*n_register) > memory.read_register(*m_register);

                self.pc += 2;
            }
            Instruction::CmpGt(n_register, m_register) => {
                memory.t = (memory.read_register(*n_register) as i32)
                    > (memory.read_register(*m_register) as i32);

                self.pc += 2;
            }
            Instruction::CmpPz(n_register) => {
                let value = memory.read_register(*n_register);
                memory.t = (value as i32) >= 0;

                self.pc += 2;
            }
            Instruction::Dt(n_register) => {
                let n_value = memory.read_register(*n_register) - 1;
                memory.t = n_value == 0;
                memory.write_register(*n_register, n_value);

                self.pc += 2;
            }

            // Control Registers
            Instruction::StsPr(register) => {
                memory.write_register(*register, memory.pr);
                self.pc += 2;
            }
            Instruction::PushPROther(n_register) => {
                memory.write_register(*n_register, memory.read_register(*n_register) - 4);
                memory.write_long(memory.read_register(*n_register), memory.pr, display);
                self.pc += 2;
            }
            Instruction::PopPROther(n_register) => {
                let value = memory.read_long(memory.read_register(*n_register));
                memory.pr = value;
                memory.write_register(*n_register, memory.read_register(*n_register) + 4);
                self.pc += 2;
            }
            Instruction::StsMacl(target) => {
                memory.write_register(*target, memory.macl);
                self.pc += 2;
            }
            Instruction::StsLMacl(n_register) => {
                memory.write_register(*n_register, memory.read_register(*n_register) - 4);
                memory.write_long(memory.read_register(*n_register), memory.macl, display);
                self.pc += 2;
            }
            Instruction::LdsLMacl(stack_reg) => {
                let value = memory.read_long(memory.read_register(*stack_reg));
                memory.macl = value;

                memory.write_register(*stack_reg, memory.read_register(*stack_reg) + 4);

                self.pc += 2;
            }
            Instruction::StsMach(target) => {
                memory.write_register(*target, memory.mach);
                self.pc += 2;
            }
            Instruction::StsLMach(n_register) => {
                memory.write_register(*n_register, memory.read_register(*n_register) - 4);
                memory.write_long(memory.read_register(*n_register), memory.mach, display);
                self.pc += 2;
            }
            Instruction::LdsLMach(stack_reg) => {
                let value = memory.read_long(memory.read_register(*stack_reg));
                memory.mach = value;

                memory.write_register(*stack_reg, memory.read_register(*stack_reg) + 4);

                self.pc += 2;
            }

            // Logic Instructions
            Instruction::Tst(n_register, m_register) => {
                let n_value = memory.read_register(*n_register);
                let m_value = memory.read_register(*m_register);
                memory.t = (n_value & m_value) == 0;

                self.pc += 2;
            }
            Instruction::Xor(n_register, m_register) => {
                memory.write_register(
                    *n_register,
                    memory.read_register(*n_register) ^ memory.read_register(*m_register),
                );
                self.pc += 2;
            }
            Instruction::Or(n_register, m_register) => {
                let n_value = memory.read_register(*n_register);
                let m_value = memory.read_register(*m_register);
                let value = n_value | m_value;
                memory.write_register(*n_register, value);

                self.pc += 2;
            }

            Instruction::Nop => {
                self.pc += 2;
            }
            _ => {
                println!("[{:4x}] Unknown Instruction: {:?}", self.pc, instr);
                return Err(Exception::UnknownInstruction);
            }
        };

        Ok(())
    }

    /// Emulates a single CPU-Tick, meaning that only one instruction will be executed
    pub fn tick<D, I>(
        &mut self,
        memory: &mut Memory,
        display: &mut D,
        input: &mut I,
        debugger: &dyn Debugger,
    ) -> Result<(), Exception>
    where
        D: Display,
        I: Input,
    {
        // If there was a instruction queued up before the current
        // one (by a delayed branch for example), execute that one first
        // and return as it will only execute one instruction per tick
        if let Some((addr, queued)) = std::mem::replace(&mut self.queued_instr, None) {
            let prev_pc = self.pc;
            debugger.print_instr(addr, &queued);
            self.execute(queued, memory, display, debugger)?;
            self.pc = prev_pc;
            return Ok(());
        }

        match self.pc {
            // Syscalls
            0x80020070 => {
                let id = memory.read_register(0);
                system::syscall(id, memory, input, display);
                self.pc = memory.pr;

                Ok(())
            }
            _ => {
                let instr = Self::fetch_instruction(self.pc, memory);
                debugger.print_instr(self.pc, &instr);
                self.execute(instr, memory, display, debugger)
            }
        }
    }
}
