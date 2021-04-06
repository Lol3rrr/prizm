use crate::{asm, g3a};

mod memory;
use memory::Memory;

const CODE_MAPPING_OFFSET: u32 = 0x00300000;

pub struct Emulator {
    memory: Memory,
    pc: u32,
}

enum InstructionType {
    Branch,
    Other,
}

#[derive(Debug)]
pub enum Exception {
    UnknownInstruction,
    SlotIllegal,
}

impl Emulator {
    pub fn new(file: g3a::File) -> Self {
        let mut memory = Memory::new();
        memory.write_register(15, 0x7FFFF);
        memory.write_register(14, 0x7FFFF);

        for (i, tmp) in file.executable_code.iter().enumerate() {
            memory.write_byte(i as u32 + CODE_MAPPING_OFFSET, *tmp);
        }

        Self {
            memory,
            pc: CODE_MAPPING_OFFSET,
        }
    }

    pub fn print_registers(&self) {
        self.memory.print_registers();
    }

    pub fn get_instr(&mut self, offset: u32) -> Option<asm::Instruction> {
        match self.fetch_instruction(self.pc + offset) {
            Ok(i) => Some(i),
            Err(_) => None,
        }
    }

    fn handle_jump(&mut self, destination: u32, delayed: bool) {
        let prev_pc = self.pc;
        if delayed {
            self.pc += 2;
            self.emulate_single().unwrap();
        }

        let n_pc = match destination {
            _ if destination == 0x80020070 => {
                let syscall = self.memory.read_register(0);
                match syscall {
                    0xeab => {
                        println!("GeyKey System-Call");
                    }
                    0x0272 => {
                        println!("Bdisp_AllClr_VRAM System-Call");
                    }
                    _ => {
                        println!("Unknown Syscall: {:x}", syscall);
                    }
                };
                prev_pc + 4
            }
            _ => destination,
        };
        self.pc = n_pc;
    }

    pub fn fetch_instruction(&mut self, pc: u32) -> Result<asm::Instruction, u16> {
        let word_bytes = self.memory.read_word(pc);
        match asm::Instruction::parse(word_bytes) {
            Some(i) => Ok(i),
            None => Err(word_bytes),
        }
    }

    fn sign_extend_u8(raw_value: u8) -> u32 {
        if (raw_value & 0x80) == 0 {
            0x000000FF & (raw_value as u32)
        } else {
            0xFFFFFF00 | (raw_value as u32)
        }
    }

    fn sign_extend_u12(raw_value: u16) -> u32 {
        if (raw_value & 0x800) == 0 {
            0x00000FFF & (raw_value as u32)
        } else {
            0xFFFFF000 | (raw_value as u32)
        }
    }

    fn fetch_instruction_type(&mut self, pc: u32) -> InstructionType {
        match self.fetch_instruction(pc) {
            Ok(asm::Instruction::BF(_))
            | Ok(asm::Instruction::BT(_))
            | Ok(asm::Instruction::BRA(_))
            | Ok(asm::Instruction::BSR(_))
            | Ok(asm::Instruction::Jmp(_))
            | Ok(asm::Instruction::Jsr(_))
            | Ok(asm::Instruction::Rts) => InstructionType::Branch,
            _ => InstructionType::Other,
        }
    }

    pub fn emulate_single(&mut self) -> Result<(), Exception> {
        match self.fetch_instruction(self.pc) {
            Ok(instr) => {
                match instr {
                    // Move Instructions
                    asm::Instruction::Mov(n_register, m_register) => {
                        println!("[{:4x}] MOV R{} -> R{}", self.pc, m_register, n_register);

                        self.memory
                            .write_register(n_register, self.memory.read_register(m_register));
                        self.pc += 2;
                    }
                    asm::Instruction::MovI(register, raw_value) => {
                        println!("[{:4x}] MOV x{:x} -> R{}", self.pc, raw_value, register);

                        let value = Self::sign_extend_u8(raw_value);
                        self.memory.write_register(register, value);
                        self.pc += 2;
                    }
                    asm::Instruction::MovL(
                        asm::Operand::Register(n_register),
                        asm::Operand::AtRegister(m_register),
                    ) => {
                        println!(
                            "[{:4x}] MOV.L (R{}) -> R{}",
                            self.pc, m_register, n_register
                        );

                        let value = self.memory.read_long(self.memory.read_register(m_register));
                        self.memory.write_register(n_register, value);
                        self.pc += 2;
                    }
                    asm::Instruction::MovL(
                        asm::Operand::AtRegister(n_register),
                        asm::Operand::Register(m_register),
                    ) => {
                        println!(
                            "[{:4x}] MOV.L R{} -> (R{})",
                            self.pc, m_register, n_register
                        );

                        self.memory.write_long(
                            self.memory.read_register(n_register),
                            self.memory.read_register(m_register),
                        );
                        self.pc += 2;
                    }
                    asm::Instruction::PopOther(n_register, m_register) => {
                        println!(
                            "[{:4x}] MOV.L (R{}) -> R{}, R{} + 4 -> R{}",
                            self.pc, m_register, n_register, m_register, m_register
                        );

                        let value = self.memory.read_long(self.memory.read_register(m_register));
                        self.memory.write_register(n_register, value);
                        self.memory
                            .write_register(m_register, self.memory.read_register(m_register) + 4);
                        self.pc += 2;
                    }
                    asm::Instruction::PushOther(m_register, n_register) => {
                        println!(
                            "[{:4x}] MOV.L R{}-4 -> R{}, R{} -> (R{})",
                            self.pc, n_register, n_register, m_register, n_register
                        );

                        let mut n = self.memory.read_register(n_register);
                        n -= 4;
                        self.memory.write_register(n_register, n);
                        self.memory
                            .write_long(n, self.memory.read_register(m_register));
                        self.pc += 2;
                    }
                    asm::Instruction::MovL(
                        asm::Operand::Register(n_register),
                        asm::Operand::Displacement8(raw_disp),
                    ) => {
                        println!("[{:4x}] MOV.L (disp*4 + (PC & 0xFFFFFFFC) + 4) -> sign extension -> R{}", self.pc, n_register);

                        let raw_immediate: u32 = 0x000000FF & (raw_disp) as u32;
                        let addr = (self.pc & 0xFFFFFFFC) + 4 + (raw_immediate * 4);
                        let data = self.memory.read_long(addr);

                        self.memory.write_register(n_register, data);
                        self.pc += 2;
                    }

                    // Shift instructions
                    asm::Instruction::Shll(n_register) => {
                        println!("[{:4x}] SHLL T << R{} << 1", self.pc, n_register);

                        let value = self.memory.read_register(n_register);
                        self.memory.t = (value & 0x80000000) != 0;

                        self.memory.write_register(n_register, value << 1);
                        self.pc += 2;
                    }
                    asm::Instruction::Shll2(register) => {
                        println!("[{:4x}] SHLL2 R{} << 2 -> R{}", self.pc, register, register);

                        let data = self.memory.read_register(register);
                        self.memory.write_register(register, data << 2);
                        self.pc += 2;
                    }
                    asm::Instruction::Shll8(register) => {
                        println!("[{:4x}] SHLL8 R{} << 8 -> R{}", self.pc, register, register);

                        let data = self.memory.read_register(register);
                        self.memory.write_register(register, data << 8);
                        self.pc += 2;
                    }
                    asm::Instruction::Shll16(register) => {
                        println!(
                            "[{:4x}] SHLL16 R{} << 16 -> R{}",
                            self.pc, register, register
                        );

                        let data = self.memory.read_register(register);
                        self.memory.write_register(register, data << 16);
                        self.pc += 2;
                    }
                    asm::Instruction::Shlr(n_register) => {
                        println!("[{:4x}] SHLR 1 >> R{} >> T", self.pc, n_register);

                        let value = self.memory.read_register(n_register);
                        self.memory.t = (value & 0x00000001) != 0;

                        self.memory.write_register(n_register, value >> 1);
                        self.pc += 2;
                    }

                    // Arithmetic
                    asm::Instruction::AddI(register, value) => {
                        let add_value = Self::sign_extend_u8(value);
                        println!(
                            "[{:4x}] Add {:x} + R{} -> R{}",
                            self.pc, value, register, register
                        );

                        self.memory.write_register(
                            register,
                            self.memory.read_register(register).wrapping_add(add_value),
                        );
                        self.pc += 2
                    }

                    // Branch Instructions
                    asm::Instruction::Jmp(register) => {
                        println!("[{:4x}] Jumping to value in R{}", self.pc, register);

                        let destination = self.memory.read_register(register);
                        println!("[{:4x}] Jump-Destination: x{:08x}", self.pc, destination);

                        self.handle_jump(destination, true);
                    }
                    asm::Instruction::BT(raw_disp) => {
                        println!("[{:4x}] BT", self.pc);

                        let disp = Self::sign_extend_u8(raw_disp) << 1;
                        if self.memory.t {
                            println!("[{:4x}] Jumping: x{:x}", self.pc, disp);
                            self.handle_jump(self.pc + 4 + disp, false);
                        } else {
                            self.pc += 2;
                        }
                    }
                    asm::Instruction::BF(raw_disp) => {
                        println!("[{:4x}] BF", self.pc);

                        let disp = Self::sign_extend_u8(raw_disp) << 1;
                        if !self.memory.t {
                            println!("[{:4x}] Jumping: x{:x}", self.pc, disp);
                            self.handle_jump(self.pc + 4 + disp, false);
                        } else {
                            self.pc += 2;
                        }
                    }
                    asm::Instruction::Jsr(m_register) => {
                        println!("[{:4x}] JSR R{} -> PC", self.pc, m_register);
                        match self.fetch_instruction_type(self.pc + 2) {
                            InstructionType::Branch => return Err(Exception::SlotIllegal),
                            _ => {}
                        };

                        let destination = self.memory.read_register(m_register);
                        println!("[{:4x}] JSR-Destination: {:08x}", self.pc, destination);
                        self.memory.pr = self.pc + 4;
                        self.handle_jump(destination, true);
                    }
                    asm::Instruction::Rts => {
                        println!("[{:4x}] RTS", self.pc);
                        match self.fetch_instruction_type(self.pc + 2) {
                            InstructionType::Branch => return Err(Exception::SlotIllegal),
                            _ => {}
                        };

                        let destination = self.memory.pr;
                        println!("[{:4x}] Returning to {:08x}", self.pc, destination);
                        self.handle_jump(destination, true);
                    }
                    asm::Instruction::BRA(raw_disp) => {
                        let disp = Self::sign_extend_u12(raw_disp) << 1;

                        println!("[{:4x}] BRA", self.pc);
                        match self.fetch_instruction_type(self.pc + 2) {
                            InstructionType::Branch => return Err(Exception::SlotIllegal),
                            _ => {}
                        };

                        let target = self.pc.wrapping_add(4).wrapping_add(disp);
                        println!("[{:4x}] Jumping x{:X} to x{:X}", self.pc, disp, target);
                        self.handle_jump(target, true);
                    }
                    asm::Instruction::BSR(raw_disp) => {
                        let disp = Self::sign_extend_u12(raw_disp) << 1;

                        println!("[{:4x}] BSR", self.pc);
                        match self.fetch_instruction_type(self.pc + 2) {
                            InstructionType::Branch => return Err(Exception::SlotIllegal),
                            _ => {}
                        };

                        self.memory.pr = self.pc + 4;
                        let target = self.pc.wrapping_add(4).wrapping_add(disp);
                        println!("[{:4x}] Jumping x{:X} to x{:X}", self.pc, disp, target);
                        self.handle_jump(target, true);
                    }

                    // Comparisons
                    asm::Instruction::CmpEq(n_register, m_register) => {
                        println!("[{:4x}] CMP/EQ R{} = R{}", self.pc, n_register, m_register);

                        self.memory.t = self.memory.read_register(n_register)
                            == self.memory.read_register(m_register);

                        self.pc += 2;
                    }
                    asm::Instruction::CmpHs(n_register, m_register) => {
                        println!(
                            "[{:4x}] CMP/HS R{} >= R{} (unsigned)",
                            self.pc, n_register, m_register
                        );

                        self.memory.t = self.memory.read_register(n_register)
                            >= self.memory.read_register(m_register);

                        self.pc += 2;
                    }
                    asm::Instruction::CmpHi(n_register, m_register) => {
                        println!(
                            "[{:4x}] CMP/HI R{} > R{} (unsigned)",
                            self.pc, n_register, m_register
                        );

                        self.memory.t = self.memory.read_register(n_register)
                            > self.memory.read_register(m_register);

                        self.pc += 2;
                    }

                    // Control Registers
                    asm::Instruction::STS(register) => {
                        println!("[{:4x}] STS PR -> R{}", self.pc, register);
                        self.memory.write_register(register, self.memory.pr);
                        self.pc += 2;
                    }
                    asm::Instruction::PushPROther(n_register) => {
                        println!(
                            "[{:4x}] STS.L R{} - 4 -> R{}, PR -> (R{})",
                            self.pc, n_register, n_register, n_register
                        );

                        self.memory
                            .write_register(n_register, self.memory.read_register(n_register) - 4);
                        self.memory
                            .write_long(self.memory.read_register(n_register), self.memory.pr);
                        self.pc += 2;
                    }

                    // Logic Instructions
                    asm::Instruction::Xor(n_register, m_register) => {
                        println!(
                            "[{:4x}] XOR R{} ^ R{} -> R{}",
                            self.pc, n_register, m_register, n_register
                        );

                        self.memory.write_register(
                            n_register,
                            self.memory.read_register(n_register)
                                ^ self.memory.read_register(m_register),
                        );
                        self.pc += 2;
                    }
                    asm::Instruction::Nop => {
                        println!("[{:4x}] NOP", self.pc);
                        self.pc += 2;
                    }
                    _ => {
                        println!("[{:4x}] Unknown Instruction: {:?}", self.pc, instr);
                        self.pc += 2;
                        //return false;
                    }
                };

                Ok(())
            }
            Err(raw) => {
                println!("[{:4x}] Unknown Instruction: x{:04X}", self.pc, raw);
                self.pc += 2;
                Err(Exception::UnknownInstruction)
            }
        }
    }
}
