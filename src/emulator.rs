use std::usize;

use crate::g3a;

mod memory;
use memory::Memory;

const CODE_MAPPING_OFFSET: u32 = 0x00300000;

fn handle_jump(pc: u32, destination: u32, mem: &Memory) -> u32 {
    match destination {
        _ if destination == 0x80020070 => {
            let syscall = mem.read_register(0);
            match syscall {
                0xeab => {
                    println!("GeyKey System-Call");
                }
                _ => {
                    println!("Unknown Syscall: {}", syscall);
                }
            };

            pc + 2
        }
        _ if destination > CODE_MAPPING_OFFSET => destination - 2 - CODE_MAPPING_OFFSET,
        _ => destination - 2,
    }
}

pub struct Emulator {
    memory: Memory,
    pc: u32,
    code: Vec<u8>,
}

impl Emulator {
    pub fn new(file: g3a::File) -> Self {
        let mut memory = Memory::new();
        memory.write_register(15, 0x7FFFF);

        Self {
            memory,
            pc: 0,
            code: file.executable_code,
        }
    }

    pub fn fetch_instruction(&self, pc: u32) -> Option<(u8, u8, u8, u8)> {
        let first = self.code.get(pc as usize)?;
        let second = self.code.get((pc + 1) as usize)?;

        Some((
            (first & 0xf0) >> 4,
            first & 0x0f,
            (second & 0xf0) >> 4,
            second & 0x0f,
        ))
    }

    pub fn emulate_single(&mut self) -> bool {
        match self.fetch_instruction(self.pc) {
            Some((nibble_1, nibble_2, nibble_3, nibble_4)) => {
                match (nibble_1, nibble_2, nibble_3, nibble_4) {
                    (0xe, register, value_p1, value_p2) => {
                        println!(
                            "[{:4x}] Moving {:x}{:x} -> R{}",
                            self.pc, value_p1, value_p2, register
                        );

                        // TODO
                        // Sign extension
                        let value = (value_p1 << 4) + value_p2;
                        self.memory.write_register(register, value as u32);
                    }
                    (0x4, register, 0x1, 0x8) => {
                        println!(
                            "[{:4x}] Shifting R{} by 8 bits to the left",
                            self.pc, register
                        );

                        let data = self.memory.read_register(register);
                        self.memory.write_register(register, data << 8);
                    }
                    (0x4, register, 0x2, 0x8) => {
                        println!(
                            "[{:4x}] Shifting R{} by 16 bits to the left",
                            self.pc, register
                        );

                        let data = self.memory.read_register(register);
                        self.memory.write_register(register, data << 16);
                    }
                    (0x7, register, value_p1, value_p2) => {
                        println!(
                            "[{:4x}] Add {:x}{:x} + R{} -> R{}",
                            self.pc, value_p1, value_p2, register, register
                        );

                        let value = ((value_p1 << 4) + value_p2) as u32;
                        let prev_data = self.memory.read_register(register);
                        self.memory.write_register(register, prev_data + value);
                    }
                    (0x4, register, 0x2, 0xb) => {
                        println!("[{:4x}] Jumping to value in R{}", self.pc, register);

                        let destination = self.memory.read_register(register);
                        println!("[{:4x}] Jump-Destination: {:08x}", self.pc, destination);
                        self.pc = handle_jump(self.pc, destination, &self.memory);
                    }
                    (0x0, 0x0, 0x0, 0x9) => {
                        println!("[{:4x}] NOP", self.pc);
                    }
                    (0x0, register, 0x2, 0xa) => {
                        println!("[{:4x}] STS PR -> R{}", self.pc, register);
                        self.memory.write_register(register, self.memory.pr);
                    }
                    (0x2, n_register, m_register, 0x2) => {
                        println!(
                            "[{:4x}] MOV.L R{} -> (R{})",
                            self.pc, m_register, n_register
                        );
                        self.memory.write_heap(
                            self.memory.read_register(n_register),
                            self.memory.read_register(m_register),
                        );
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
                            .write_heap(n, self.memory.read_register(m_register));
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
                    }
                    (0x4, m_register, 0x0, 0xb) => {
                        println!("[{:4x}] JSR R{} -> PC", self.pc, m_register);

                        let destination = self.memory.read_register(m_register);
                        println!("[{:4x}] JSR-Destination: {:08x}", self.pc, destination);
                        self.memory.pr = self.pc + 4;
                        self.pc = handle_jump(self.pc, destination, &self.memory);
                    }
                    (0x0, 0x0, 0x0, 0xb) => {
                        println!("[{:4x}] RTS", self.pc);

                        let destination = self.memory.pr;
                        println!("[{:4x}] Returning to {:08x}", self.pc, destination);
                        self.pc = handle_jump(self.pc, destination, &self.memory);
                    }
                    (0x6, n_register, m_register, 0x2) => {
                        println!(
                            "[{:4x}] MOV.L (R{}) -> R{}",
                            self.pc, m_register, n_register
                        );

                        self.memory.write_register(
                            n_register,
                            self.memory.read_heap(self.memory.read_register(m_register)),
                        );
                    }
                    (0x6, n_register, m_register, 0x6) => {
                        println!(
                            "[{:4x}] MOV.L (R{}) -> R{}, R{} + 4 -> R{}",
                            self.pc, m_register, n_register, m_register, m_register
                        );

                        self.memory.write_register(
                            n_register,
                            self.memory.read_heap(self.memory.read_register(m_register)),
                        );
                        self.memory
                            .write_register(m_register, self.memory.read_register(m_register) + 4);
                    }
                    (0x4, m_register, 0x2, 0xa) => {
                        println!("[{:4x}] lds R{} -> PR", self.pc, m_register);

                        self.memory.pr = self.memory.read_register(m_register);
                    }
                    (0x3, n_register, m_register, 0x0) => {
                        println!("[{:4x}] CMP/EQ R{} = R{}", self.pc, n_register, m_register);

                        if self.memory.read_register(n_register)
                            == self.memory.read_register(m_register)
                        {
                            self.memory.t = 1;
                        } else {
                            self.memory.t = 0;
                        }
                    }
                    (0xa, d_1, d_2, d_3) => {
                        let raw_disp: u16 = 0x0fff
                            & (((0x000f & d_1 as u16) << 8)
                                | (((0x000f & d_2 as u16) << 4) | (0x000f & d_3 as u16)));
                        let (disp, sub) = if (raw_disp & 0x800) == 0 {
                            ((0x00000fff & (raw_disp as u32)) * 2, false)
                        } else {
                            let tmp = raw_disp as u32;
                            (((tmp - 1) ^ 0x00000fff) * 2, true)
                        };

                        println!("[{:4x}] BRA", self.pc);
                        if !sub {
                            println!("[{:4x}] Jumping Forward: {:x}", self.pc, disp);
                            self.pc = self.pc + 2 + disp;
                        } else {
                            println!("[{:4x}] Jumping Backwards: {:x}", self.pc, disp);
                            self.pc = self.pc + 2 - disp;
                        }
                    }
                    (0x8, 0x9, d_1, d_2) => {
                        println!("[{:4x}] BT", self.pc);

                        let raw_disp = (d_1 << 4) | (d_2 & 0x0f);
                        let disp = if (raw_disp & 0x80) == 0 {
                            0x000000ff & raw_disp as u32
                        } else {
                            unimplemented!("Cannot jump back yet");
                        } * 2;

                        if self.memory.t == 1 {
                            println!("[{:4x}] Jumping: {:x}", self.pc, disp);
                            self.pc = self.pc + 2 + disp;
                        }
                    }
                    _ => {
                        println!(
                            "[{:4x}] Unknown Instruction: {:x}{:x}{:x}{:x}",
                            self.pc, nibble_1, nibble_2, nibble_3, nibble_4
                        );
                        return false;
                    }
                };

                self.pc += 2;
                true
            }
            _ => false,
        }
    }
}
