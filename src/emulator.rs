use crate::g3a;

pub struct Memory {
    registers: [u32; 16],
}
impl Memory {
    pub fn new() -> Self {
        Self { registers: [0; 16] }
    }

    pub fn write_register(&mut self, reg: u8, value: u32) {
        if reg > 15 {
            panic!("Invalid Register Access: {}", reg);
        }

        *self.registers.get_mut(reg as usize).unwrap() = value;
    }
    pub fn read_register(&self, reg: u8) -> u32 {
        if reg > 15 {
            panic!("Invalid Register Access: {}", reg);
        }

        *self.registers.get(reg as usize).unwrap()
    }
}

pub fn emulate(file: g3a::File) {
    let code = file.executable_code;

    let mut mem = Memory::new();

    let mut pc: u32 = 0;
    loop {
        match code.get(pc as usize) {
            Some(part1) => {
                let part2 = code.get((pc + 1) as usize).unwrap();

                let nibble_1 = (part1 & 0xf0) >> 4;
                let nibble_2 = part1 & 0x0f;
                let nibble_3 = (part2 & 0xf0) >> 4;
                let nibble_4 = part2 & 0x0f;

                match (nibble_1, nibble_2, nibble_3, nibble_4) {
                    (0xe, register, value_p1, value_p2) => {
                        println!(
                            "[{:4x}] Moving {:x}{:x} -> R{}",
                            pc, value_p1, value_p2, register
                        );

                        // TODO
                        // Sign extension
                        let value = (value_p1 << 4) + value_p2;
                        mem.write_register(register, value as u32);
                    }
                    (0x4, register, 0x1, 0x8) => {
                        println!("[{:4x}] Shifting R{} by 8 bits to the left", pc, register);

                        let data = mem.read_register(register);
                        mem.write_register(register, data << 8);
                    }
                    (0x4, register, 0x2, 0x8) => {
                        println!("[{:4x}] Shifting R{} by 16 bits to the left", pc, register);

                        let data = mem.read_register(register);
                        mem.write_register(register, data << 16);
                    }
                    (0x7, register, value_p1, value_p2) => {
                        println!(
                            "[{:4x}] Add {:x}{:x} + R{} -> R{}",
                            pc, value_p1, value_p2, register, register
                        );

                        let value = ((value_p1 << 4) + value_p2) as u32;
                        let prev_data = mem.read_register(register);
                        mem.write_register(register, prev_data + value);
                    }
                    (0x4, register, 0x2, 0xb) => {
                        println!("[{:4x}] Jumping to value in R{}", pc, register);

                        let destination = mem.read_register(register);
                        println!("[{:4x}] Jump-Destination: {:08x}", pc, destination);
                        pc = destination - 2;
                    }
                    (0x0, 0x0, 0x0, 0x9) => {
                        println!("[{:4x}] NOP", pc);
                    }
                    _ => {
                        println!(
                            "[{:4x}] Unknown Instruction: {:x}{:x}{:x}{:x}",
                            pc, nibble_1, nibble_2, nibble_3, nibble_4
                        );
                        break;
                    }
                };

                pc += 2;
            }
            _ => {
                break;
            }
        };
    }
}
