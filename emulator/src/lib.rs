mod memory;
pub use memory::Memory;

mod input;
pub use input::MockInput;

mod instructiontype;
use instructiontype::InstructionType;

use sh::asm;

pub const CODE_MAPPING_OFFSET: u32 = 0x00300000;

pub struct Emulator<'a, K>
where
    K: Input,
{
    file: Option<g3a::File>,
    memory: Memory,
    pc: u32,
    input: &'a mut K,
    debug: bool,
}

#[derive(Debug)]
pub enum Exception {
    UnknownInstruction,
    SlotIllegal,
}

pub trait Input {
    fn get_key(&mut self);
}

impl<'a, K> Emulator<'a, K>
where
    K: Input,
{
    pub fn new<'b>(file: g3a::File, input: &'b mut K) -> Self
    where
        'b: 'a,
    {
        let mut memory = Memory::new();
        memory.write_register(15, 0x80000);
        memory.write_register(14, 0x80000);

        for (i, tmp) in file.executable_code.iter().enumerate() {
            memory.write_byte(i as u32 + CODE_MAPPING_OFFSET, *tmp);
        }

        Self {
            file: Some(file),
            memory,
            pc: CODE_MAPPING_OFFSET,
            input,
            debug: false,
        }
    }

    pub fn new_test<'b>(input: &'b mut K, instructions: Vec<asm::Instruction>) -> Self
    where
        'b: 'a,
    {
        let mut memory = Memory::new();
        memory.write_register(15, 0x80000);
        memory.write_register(14, 0x80000);

        let mut index = 0;
        for instr in instructions.iter() {
            let data = instr.to_byte();
            memory.write_byte(index + CODE_MAPPING_OFFSET, data[0]);
            memory.write_byte(index + CODE_MAPPING_OFFSET + 1, data[1]);

            index += 2;
        }

        Self {
            file: None,
            memory,
            pc: CODE_MAPPING_OFFSET,
            input,
            debug: true,
        }
    }
    pub fn new_test_raw<'b>(input: &'b mut K, instr: Vec<u8>, mut memory: Memory) -> Self
    where
        'b: 'a,
    {
        for (index, byte) in instr.iter().enumerate() {
            memory.write_byte(index as u32 + CODE_MAPPING_OFFSET, *byte);
        }

        Self {
            file: None,
            memory,
            pc: CODE_MAPPING_OFFSET,
            input,
            debug: true,
        }
    }

    pub fn set_verbose(&mut self, val: bool) {
        self.debug = val;
    }

    pub fn pc(&self) -> u32 {
        self.pc
    }

    pub fn clone_registers(&self) -> [u32; 16] {
        self.memory.clone_registers()
    }
    pub fn clone_heap(&self) -> Vec<u8> {
        self.memory.clone_heap()
    }

    pub fn print_code(&mut self, p_start: Option<usize>, p_end: Option<usize>) {
        let file = self.file.as_ref().unwrap();

        let start = p_start.unwrap_or(0);
        let end = p_end.unwrap_or(file.executable_code.len() - 1);
        let code_len = file.executable_code.len();

        assert!(start < code_len);
        assert!(end < code_len);
        assert!((start % 2) == 0);

        for offset in start..end {
            if offset % 2 != 0 {
                continue;
            }
            let index = start + offset + CODE_MAPPING_OFFSET as usize;
            let instr = self.fetch_instruction(index as u32);

            println!("[{:4x}] {:?}", index, instr);
        }
    }

    pub fn print_registers(&self) {
        self.memory.print_registers();
    }

    pub fn print_stack(&self) {
        let start = self.memory.read_register(15) + 1;
        let end = self.memory.read_register(14);

        self.memory.print_memory(start as usize, end as usize);
    }

    pub fn get_instr(&mut self, offset: u32) -> asm::Instruction {
        self.fetch_instruction(self.pc + offset)
    }

    fn handle_jump(&mut self, destination: u32, delayed: bool) {
        if delayed {
            self.pc += 2;
            self.emulate_single().unwrap();
        }

        let n_pc = match destination {
            _ if destination == 0x80020070 => {
                let syscall = self.memory.read_register(0);
                match syscall {
                    0xeab => {
                        println!("Get-Key");
                        self.input.get_key();
                    }
                    0x0272 => {
                        println!("Bdisp_AllClr_VRAM System-Call");
                    }
                    _ => {
                        println!("Unknown Syscall: {:x}", syscall);
                    }
                };
                self.memory.pr
            }
            _ => destination,
        };
        self.pc = n_pc;
    }

    pub fn fetch_instruction(&mut self, pc: u32) -> asm::Instruction {
        let word_bytes = self.memory.read_word(pc);
        asm::Instruction::parse(word_bytes)
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
        InstructionType::parse(self.fetch_instruction(pc))
    }

    fn print_instr(&self, instr: &asm::Instruction) {
        if self.debug {
            println!("[{:4x}] {:?}", self.pc, instr);
        }
    }

    pub fn run_until(&mut self, target_pc: u32) -> Result<(), Exception> {
        loop {
            self.emulate_single()?;
            if self.pc() >= target_pc {
                return Ok(());
            }
        }
    }
    pub fn run_completion(&mut self) -> Result<(), Exception> {
        loop {
            self.emulate_single()?;

            if self.pc == 0 {
                return Ok(());
            }
        }
    }

    pub fn emulate_single(&mut self) -> Result<(), Exception> {
        let instr = self.fetch_instruction(self.pc);

        match instr {
            // Move Instructions
            asm::Instruction::Mov(n_register, m_register) => {
                self.print_instr(&instr);

                self.memory
                    .write_register(n_register, self.memory.read_register(m_register));
                self.pc += 2;
            }
            asm::Instruction::MovI(register, raw_value) => {
                self.print_instr(&instr);

                let value = Self::sign_extend_u8(raw_value);
                self.memory.write_register(register, value);
                self.pc += 2;
            }
            asm::Instruction::MovL(
                asm::Operand::Register(n_register),
                asm::Operand::AtRegister(m_register),
            ) => {
                self.print_instr(&instr);

                let value = self.memory.read_long(self.memory.read_register(m_register));
                self.memory.write_register(n_register, value);
                self.pc += 2;
            }
            asm::Instruction::MovB(
                asm::Operand::AtRegister(n_register),
                asm::Operand::Register(m_register),
            ) => {
                self.print_instr(&instr);

                let target_addr = self.memory.read_register(n_register);
                let value = self.memory.read_register(m_register) as u8;
                self.memory.write_byte(target_addr, value);
                self.pc += 2;
            }
            asm::Instruction::MovW(
                asm::Operand::AtRegister(n_register),
                asm::Operand::Register(m_register),
            ) => {
                self.print_instr(&instr);

                let target_addr = self.memory.read_register(n_register);
                let value = self.memory.read_register(m_register) as u16;
                self.memory.write_word(target_addr, value);
                self.pc += 2;
            }
            asm::Instruction::MovL(
                asm::Operand::AtRegister(n_register),
                asm::Operand::Register(m_register),
            ) => {
                self.print_instr(&instr);

                let addr = self.memory.read_register(n_register);
                let value = self.memory.read_register(m_register);
                self.memory.write_long(addr, value);
                self.pc += 2;
            }
            asm::Instruction::PopOther(n_register, m_register) => {
                self.print_instr(&instr);

                let address = self.memory.read_register(m_register);
                let value = self.memory.read_long(address);
                self.memory.write_register(n_register, value);
                self.memory.write_register(m_register, address + 4);
                self.pc += 2;
            }
            asm::Instruction::PushOther(m_register, n_register) => {
                self.print_instr(&instr);

                let mut n = self.memory.read_register(n_register);
                n -= 4;
                self.memory.write_register(n_register, n);
                self.memory
                    .write_long(n, self.memory.read_register(m_register));
                self.pc += 2;
            }
            asm::Instruction::MovW(
                asm::Operand::Register(n_register),
                asm::Operand::Displacement8(raw_disp),
            ) => {
                self.print_instr(&instr);

                match self.fetch_instruction_type(self.pc + 2) {
                    InstructionType::Branch => return Err(Exception::SlotIllegal),
                    _ => {}
                };

                let raw_immediate: u32 = 0x000000FF & (raw_disp as u32);
                let addr = self.pc + 4 + (raw_immediate * 2);
                let data = self.memory.read_word(addr);

                self.memory.write_register(n_register, data as u32);
                self.pc += 2;
            }
            asm::Instruction::MovL(
                asm::Operand::Register(n_register),
                asm::Operand::Displacement8(raw_disp),
            ) => {
                self.print_instr(&instr);

                match self.fetch_instruction_type(self.pc + 2) {
                    InstructionType::Branch => return Err(Exception::SlotIllegal),
                    _ => {}
                };

                let raw_immediate: u32 = 0x000000FF & (raw_disp) as u32;
                let addr = (self.pc & 0xFFFFFFFC) + 4 + (raw_immediate * 4);
                let data = self.memory.read_long(addr);

                self.memory.write_register(n_register, data);
                self.pc += 2;
            }

            // Shift instructions
            asm::Instruction::Shll(n_register) => {
                self.print_instr(&instr);

                let value = self.memory.read_register(n_register);
                self.memory.t = (value & 0x80000000) != 0;

                self.memory.write_register(n_register, value << 1);
                self.pc += 2;
            }
            asm::Instruction::Shll2(register) => {
                self.print_instr(&instr);

                let data = self.memory.read_register(register);
                self.memory.write_register(register, data << 2);
                self.pc += 2;
            }
            asm::Instruction::Shll8(register) => {
                self.print_instr(&instr);

                let data = self.memory.read_register(register);
                self.memory.write_register(register, data << 8);
                self.pc += 2;
            }
            asm::Instruction::Shll16(register) => {
                self.print_instr(&instr);

                let data = self.memory.read_register(register);
                self.memory.write_register(register, data << 16);
                self.pc += 2;
            }
            asm::Instruction::Shlr(n_register) => {
                self.print_instr(&instr);

                let value = self.memory.read_register(n_register);
                self.memory.t = (value & 0x00000001) != 0;

                self.memory.write_register(n_register, value >> 1);
                self.pc += 2;
            }
            asm::Instruction::Shlr2(register) => {
                self.print_instr(&instr);

                let data = self.memory.read_register(register);
                self.memory.write_register(register, data >> 2);
                self.pc += 2;
            }
            asm::Instruction::Shlr8(register) => {
                self.print_instr(&instr);

                let data = self.memory.read_register(register);
                self.memory.write_register(register, data >> 8);
                self.pc += 2;
            }
            asm::Instruction::Shlr16(register) => {
                self.print_instr(&instr);

                let data = self.memory.read_register(register);
                self.memory.write_register(register, data >> 16);
                self.pc += 2;
            }

            // Arithmetic
            asm::Instruction::Add(target, other) => {
                self.print_instr(&instr);

                let target_value = self.memory.read_register(target);
                let other_value = self.memory.read_register(other);
                let final_value = target_value.wrapping_add(other_value);

                self.memory.write_register(target, final_value);
                self.pc += 2;
            }
            asm::Instruction::AddI(register, value) => {
                self.print_instr(&instr);
                let add_value = Self::sign_extend_u8(value);

                self.memory.write_register(
                    register,
                    self.memory.read_register(register).wrapping_add(add_value),
                );
                self.pc += 2
            }
            asm::Instruction::MulL(first, second) => {
                self.print_instr(&instr);

                let first_data = self.memory.read_register(first);
                let second_data = self.memory.read_register(second);

                self.memory.macl = first_data.wrapping_mul(second_data);

                self.pc += 2;
            }

            // Branch Instructions
            asm::Instruction::Jmp(register) => {
                self.print_instr(&instr);
                match self.fetch_instruction_type(self.pc + 2) {
                    InstructionType::Branch => return Err(Exception::SlotIllegal),
                    _ => {}
                };

                let destination = self.memory.read_register(register);

                self.handle_jump(destination, true);
            }
            asm::Instruction::BT(raw_disp) => {
                self.print_instr(&instr);

                let disp = Self::sign_extend_u8(raw_disp) << 1;
                if self.memory.t {
                    self.handle_jump(self.pc + 4 + disp, false);
                } else {
                    self.pc += 2;
                }
            }
            asm::Instruction::BF(raw_disp) => {
                self.print_instr(&instr);

                let disp = Self::sign_extend_u8(raw_disp) << 1;
                if !self.memory.t {
                    self.handle_jump(self.pc + 4 + disp, false);
                } else {
                    self.pc += 2;
                }
            }
            asm::Instruction::Jsr(m_register) => {
                self.print_instr(&instr);
                match self.fetch_instruction_type(self.pc + 2) {
                    InstructionType::Branch => return Err(Exception::SlotIllegal),
                    _ => {}
                };

                let destination = self.memory.read_register(m_register);
                self.memory.pr = self.pc + 4;
                self.handle_jump(destination, true);
            }
            asm::Instruction::Rts => {
                self.print_instr(&instr);
                match self.fetch_instruction_type(self.pc + 2) {
                    InstructionType::Branch => return Err(Exception::SlotIllegal),
                    _ => {}
                };

                let destination = self.memory.pr;
                self.handle_jump(destination, true);
            }
            asm::Instruction::BRA(raw_disp) => {
                self.print_instr(&instr);
                let disp = Self::sign_extend_u12(raw_disp) << 1;

                match self.fetch_instruction_type(self.pc + 2) {
                    InstructionType::Branch => return Err(Exception::SlotIllegal),
                    _ => {}
                };

                let target = self.pc.wrapping_add(4).wrapping_add(disp);
                self.handle_jump(target, true);
            }
            asm::Instruction::BSR(raw_disp) => {
                self.print_instr(&instr);
                let disp = Self::sign_extend_u12(raw_disp) << 1;

                match self.fetch_instruction_type(self.pc + 2) {
                    InstructionType::Branch => return Err(Exception::SlotIllegal),
                    _ => {}
                };

                self.memory.pr = self.pc + 4;
                let target = self.pc.wrapping_add(4).wrapping_add(disp);
                self.handle_jump(target, true);
            }

            // Comparisons
            asm::Instruction::CmpEq(n_register, m_register) => {
                self.print_instr(&instr);

                self.memory.t =
                    self.memory.read_register(n_register) == self.memory.read_register(m_register);

                self.pc += 2;
            }
            asm::Instruction::CmpHs(n_register, m_register) => {
                self.print_instr(&instr);

                self.memory.t =
                    self.memory.read_register(n_register) >= self.memory.read_register(m_register);

                self.pc += 2;
            }
            asm::Instruction::CmpHi(n_register, m_register) => {
                self.print_instr(&instr);

                self.memory.t =
                    self.memory.read_register(n_register) > self.memory.read_register(m_register);

                self.pc += 2;
            }

            // Control Registers
            asm::Instruction::STS(register) => {
                self.print_instr(&instr);

                self.memory.write_register(register, self.memory.pr);
                self.pc += 2;
            }
            asm::Instruction::PushPROther(n_register) => {
                self.print_instr(&instr);

                self.memory
                    .write_register(n_register, self.memory.read_register(n_register) - 4);
                self.memory
                    .write_long(self.memory.read_register(n_register), self.memory.pr);
                self.pc += 2;
            }
            asm::Instruction::PopPROther(n_register) => {
                self.print_instr(&instr);

                let value = self.memory.read_long(self.memory.read_register(n_register));
                self.memory.pr = value;
                self.memory
                    .write_register(n_register, self.memory.read_register(n_register) + 4);
                self.pc += 2;
            }
            asm::Instruction::StsMacl(target) => {
                self.print_instr(&instr);

                self.memory.write_register(target, self.memory.macl);
                self.pc += 2;
            }

            // Logic Instructions
            asm::Instruction::Xor(n_register, m_register) => {
                self.print_instr(&instr);

                self.memory.write_register(
                    n_register,
                    self.memory.read_register(n_register) ^ self.memory.read_register(m_register),
                );
                self.pc += 2;
            }
            asm::Instruction::Nop => {
                self.print_instr(&instr);
                self.pc += 2;
            }
            _ => {
                println!("[{:4x}] Unknown Instruction: {:?}", self.pc, instr);
                return Err(Exception::UnknownInstruction);
            }
        };

        Ok(())
    }
}
