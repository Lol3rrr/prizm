use crate::g3a;

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

    pub fn get_instr(&mut self, offset: u32) -> Option<(u8, u8, u8, u8)> {
        self.fetch_instruction(self.pc + offset)
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
                self.memory.print_registers();
                prev_pc + 4
            }
            _ => destination,
        };
        self.pc = n_pc;
    }

    pub fn fetch_instruction(&mut self, pc: u32) -> Option<(u8, u8, u8, u8)> {
        let word_bytes = self.memory.read_word(pc).to_be_bytes();
        let first = word_bytes[0];
        let second = word_bytes[1];

        Some((
            (first & 0xf0) >> 4,
            first & 0x0f,
            (second & 0xf0) >> 4,
            second & 0x0f,
        ))
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
            Some((0x8, 0xb, _, _)) // bf
            | Some((0x8, 0xf, _, _)) // bf/s
            | Some((0x8, 0x9, _, _)) // bt
            | Some((0x8, 0xd, _, _)) // bt/s
            | Some((0xa, _, _, _)) // bra
            | Some((0x0, _, 0x2, 0x3)) // braf
            | Some((0xb, _, _, _)) // bsr
            | Some((0x0, _, 0x0, 0x3)) // bsrf
            | Some((0x4, _, 0x2, 0xb)) // jmp
            | Some((0x4, _, 0x0, 0xb)) // jsr
            | Some((0x0, 0x0, 0x0, 0xb)) => InstructionType::Branch,
            _ => InstructionType::Other,
        }
    }

    pub fn emulate_single(&mut self) -> Result<(), Exception> {
        match self.fetch_instruction(self.pc) {
            Some((nibble_1, nibble_2, nibble_3, nibble_4)) => {
                match (nibble_1, nibble_2, nibble_3, nibble_4) {
                    // Move Instructions
                    (0x6, n_register, m_register, 0x3) => {
                        println!("[{:4x}] MOV R{} -> R{}", self.pc, m_register, n_register);

                        self.memory
                            .write_register(n_register, self.memory.read_register(m_register));
                        self.pc += 2;
                    }
                    (0xe, register, value_p1, value_p2) => {
                        let raw_value = (value_p1 << 4) + value_p2;
                        println!("[{:4x}] MOV x{:x} -> R{}", self.pc, raw_value, register);

                        let value = Self::sign_extend_u8(raw_value);
                        self.memory.write_register(register, value);
                        self.pc += 2;
                    }
                    (0x6, n_register, m_register, 0x2) => {
                        println!(
                            "[{:4x}] MOV.L (R{}) -> R{}",
                            self.pc, m_register, n_register
                        );

                        let value = self.memory.read_long(self.memory.read_register(m_register));
                        self.memory.write_register(n_register, value);
                        self.pc += 2;
                    }
                    (0x2, n_register, m_register, 0x2) => {
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
                    (0x6, n_register, m_register, 0x6) => {
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
                    (0x2, n_register, m_register, 0x6) => {
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
                    (0xd, n_register, d_1, d_2) => {
                        println!("[{:4x}] MOV.L (disp*4 + (PC & 0xFFFFFFFC) + 4) -> sign extension -> R{}", self.pc, n_register);

                        let raw_immediate: u32 = 0x000000FF & ((d_1 << 4) | d_2) as u32;
                        let addr = (self.pc & 0xFFFFFFFC) + 4 + (raw_immediate * 4);
                        let data = self.memory.read_long(addr);

                        match self.fetch_instruction_type(self.pc + 2) {
                            InstructionType::Branch => return Err(Exception::SlotIllegal),
                            _ => {}
                        };

                        self.memory.write_register(n_register, data);
                        self.pc += 2;
                    }

                    // Shift instructions
                    (0x4, n_register, 0x0, 0x0) => {
                        println!("[{:4x}] SHLL T << R{} << 1", self.pc, n_register);

                        let value = self.memory.read_register(n_register);
                        self.memory.t = (value & 0x80000000) != 0;

                        self.memory.write_register(n_register, value << 1);
                        self.pc += 2;
                    }
                    (0x4, register, 0x0, 0x8) => {
                        println!("[{:4x}] SHLL2 R{} << 2 -> R{}", self.pc, register, register);

                        let data = self.memory.read_register(register);
                        self.memory.write_register(register, data << 2);
                        self.pc += 2;
                    }
                    (0x4, register, 0x1, 0x8) => {
                        println!("[{:4x}] SHLL8 R{} << 8 -> R{}", self.pc, register, register);

                        let data = self.memory.read_register(register);
                        self.memory.write_register(register, data << 8);
                        self.pc += 2;
                    }
                    (0x4, register, 0x2, 0x8) => {
                        println!(
                            "[{:4x}] SHLL16 R{} << 16 -> R{}",
                            self.pc, register, register
                        );

                        let data = self.memory.read_register(register);
                        self.memory.write_register(register, data << 16);
                        self.pc += 2;
                    }
                    (0x4, n_register, 0x0, 0x1) => {
                        println!("[{:4x}] SHLR 1 >> R{} >> T", self.pc, n_register);

                        let value = self.memory.read_register(n_register);
                        self.memory.t = (value & 0x00000001) != 0;

                        self.memory.write_register(n_register, value >> 1);
                        self.pc += 2;
                    }

                    // Arithmetic
                    (0x7, register, value_p1, value_p2) => {
                        let value = (value_p1 << 4) + value_p2;
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
                    (0x4, register, 0x2, 0xb) => {
                        println!("[{:4x}] Jumping to value in R{}", self.pc, register);

                        let destination = self.memory.read_register(register);
                        println!("[{:4x}] Jump-Destination: x{:08x}", self.pc, destination);

                        self.handle_jump(destination, true);
                    }
                    (0x8, 0x9, d_1, d_2) => {
                        println!("[{:4x}] BT", self.pc);

                        let raw_disp = (d_1 << 4) | (d_2 & 0x0f);
                        let disp = if (raw_disp & 0x80) == 0 {
                            0x000000ff & raw_disp as u32
                        } else {
                            unimplemented!("Cannot jump back yet");
                        } * 2;

                        if self.memory.t {
                            println!("[{:4x}] Jumping: x{:x}", self.pc, disp);
                            self.handle_jump(self.pc + 4 + disp, false);
                        } else {
                            self.pc += 2;
                        }
                    }
                    (0x8, 0xb, d_1, d_2) => {
                        println!("[{:4x}] BF", self.pc);

                        let raw_disp = (d_1 << 4) | (d_2 & 0x0f);
                        let disp = if (raw_disp & 0x80) == 0 {
                            0x000000ff & raw_disp as u32
                        } else {
                            unimplemented!("Cannot jump back yet");
                        } * 2;

                        if !self.memory.t {
                            println!("[{:4x}] Jumping: x{:x}", self.pc, disp);
                            self.handle_jump(self.pc + 4 + disp, false);
                        } else {
                            self.pc += 2;
                        }
                    }
                    (0x4, m_register, 0x0, 0xb) => {
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
                    (0x0, 0x0, 0x0, 0xb) => {
                        println!("[{:4x}] RTS", self.pc);
                        match self.fetch_instruction_type(self.pc + 2) {
                            InstructionType::Branch => return Err(Exception::SlotIllegal),
                            _ => {}
                        };

                        let destination = self.memory.pr;
                        println!("[{:4x}] Returning to {:08x}", self.pc, destination);
                        self.handle_jump(destination, true);
                    }
                    (0xa, d_1, d_2, d_3) => {
                        let raw_disp: u16 = 0x0fff
                            & (((0x000f & d_1 as u16) << 8)
                                | (((0x000f & d_2 as u16) << 4) | (0x000f & d_3 as u16)));
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
                    (0xb, d_1, d_2, d_3) => {
                        let raw_disp: u16 = 0x0fff
                            & (((0x000f & d_1 as u16) << 8)
                                | (((0x000f & d_2 as u16) << 4) | (0x000f & d_3 as u16)));
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
                    (0x3, n_register, m_register, 0x0) => {
                        println!("[{:4x}] CMP/EQ R{} = R{}", self.pc, n_register, m_register);

                        self.memory.t = self.memory.read_register(n_register)
                            == self.memory.read_register(m_register);

                        self.pc += 2;
                    }
                    (0x3, n_register, m_register, 0x2) => {
                        println!(
                            "[{:4x}] CMP/HS R{} >= R{} (unsigned)",
                            self.pc, n_register, m_register
                        );

                        self.memory.t = self.memory.read_register(n_register)
                            >= self.memory.read_register(m_register);

                        self.pc += 2;
                    }
                    (0x3, n_register, m_register, 0x6) => {
                        println!(
                            "[{:4x}] CMP/HI R{} > R{} (unsigned)",
                            self.pc, n_register, m_register
                        );

                        self.memory.t = self.memory.read_register(n_register)
                            > self.memory.read_register(m_register);

                        self.pc += 2;
                    }

                    // Control Registers
                    (0x0, register, 0x2, 0xa) => {
                        println!("[{:4x}] STS PR -> R{}", self.pc, register);
                        self.memory.write_register(register, self.memory.pr);
                        self.pc += 2;
                    }
                    (0x4, n_register, 0x2, 0x2) => {
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
                    (0x2, n_register, m_register, 0xa) => {
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
                    (0x0, 0x0, 0x0, 0x9) => {
                        println!("[{:4x}] NOP", self.pc);
                        self.pc += 2;
                    }
                    _ => {
                        println!(
                            "[{:4x}] Unknown Instruction: {:x}{:x}{:x}{:x}",
                            self.pc, nibble_1, nibble_2, nibble_3, nibble_4
                        );
                        self.pc += 2;
                        //return false;
                    }
                };

                Ok(())
            }
            _ => Err(Exception::UnknownInstruction),
        }
    }
}
