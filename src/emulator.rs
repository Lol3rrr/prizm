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
        Self {
            memory: Memory::new(),
            pc: 0,
            code: file.executable_code,
        }
    }

    pub fn emulate_single(&mut self) -> bool {
        match self.code.get(self.pc as usize) {
            Some(part1) => {
                let part2 = self.code.get((self.pc + 1) as usize).unwrap();

                let nibble_1 = (part1 & 0xf0) >> 4;
                let nibble_2 = part1 & 0x0f;
                let nibble_3 = (part2 & 0xf0) >> 4;
                let nibble_4 = part2 & 0x0f;

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
                    (0x4, m_register, 0x2, 0xa) => {
                        println!("[{:4x}] lds R{} -> PR", self.pc, m_register);

                        self.memory.pr = self.memory.read_register(m_register);
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
