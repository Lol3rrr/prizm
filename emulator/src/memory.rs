use crate::{Display, DisplayBits};

const VRAM: u32 = 0xAC000000;

pub struct Memory {
    registers: [u32; 16],
    pub pr: u32,
    pub t: bool,
    pub macl: u32,
    pub mach: u32,
    heap: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
            pr: 0,
            t: false,
            macl: 0,
            mach: 0,
            heap: Vec::new(),
        }
    }
    pub fn print_registers(&self) {
        print!("Registers:");
        for reg in self.registers.iter() {
            print!(" x{:X},", reg);
        }
        println!();
    }
    pub fn print_memory(&self, start: usize, end: usize) {
        println!("Memory:");
        for index in start..=end {
            println!("[{:4x}] x{:X}", index, self.heap.get(index).unwrap());
        }
        println!();
    }

    pub fn clone_registers(&self) -> [u32; 16] {
        self.registers.clone()
    }
    pub fn clone_heap(&self) -> Vec<u8> {
        self.heap.clone()
    }

    fn write_u8<D>(&mut self, addr: u32, value: u8, disp: &mut D)
    where
        D: Display,
    {
        if addr >= VRAM {
            let raw_pixel_addr = addr - VRAM;
            let y = (raw_pixel_addr / 2) / 384;
            let x = (raw_pixel_addr / 2) - y * 384;
            let part = if raw_pixel_addr % 2 == 0 {
                DisplayBits::HighBits
            } else {
                DisplayBits::LowBits
            };
            disp.write_vram_u8(x, y, part, value);
            return;
        }

        let addr_u = addr as usize;
        self.check_access(addr_u, 1);
        self.heap[addr_u] = value;
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

    fn check_access(&mut self, addr: usize, size: usize) {
        let expected_size = addr + size + 1;
        if self.heap.len() < expected_size {
            self.heap.resize(expected_size, 0);
        }
    }
    pub fn write_long<D>(&mut self, addr: u32, value: u32, disp: &mut D)
    where
        D: Display,
    {
        if addr & 0x3 > 0 {
            panic!("Unaligned Long-Write to Address: x{:X}", addr);
        }

        let bytes = value.to_be_bytes();
        self.write_u8(addr, bytes[0], disp);
        self.write_u8(addr + 1, bytes[1], disp);
        self.write_u8(addr + 2, bytes[2], disp);
        self.write_u8(addr + 3, bytes[3], disp);
    }
    pub fn read_long(&mut self, addr: u32) -> u32 {
        let addr_u = addr as usize;
        self.check_access(addr_u, 4);

        let byte_1 = self.heap.get(addr_u).unwrap();
        let byte_2 = self.heap.get(addr_u + 1).unwrap();
        let byte_3 = self.heap.get(addr_u + 2).unwrap();
        let byte_4 = self.heap.get(addr_u + 3).unwrap();

        let result = u32::from_be_bytes([*byte_1, *byte_2, *byte_3, *byte_4]);

        result
    }

    pub fn write_word<D>(&mut self, addr: u32, value: u16, disp: &mut D)
    where
        D: Display,
    {
        if addr & 0x1 > 0 {
            panic!("Unaligned Word-Write to Address: x{:X}", addr);
        }

        let bytes = value.to_be_bytes();
        self.write_u8(addr, bytes[0], disp);
        self.write_u8(addr + 1, bytes[1], disp);
    }
    pub fn read_word(&mut self, addr: u32) -> u16 {
        let addr_u = addr as usize;
        self.check_access(addr_u, 2);

        let byte_1 = self.heap.get(addr_u).unwrap();
        let byte_2 = self.heap.get(addr_u + 1).unwrap();

        u16::from_be_bytes([*byte_1, *byte_2])
    }

    pub fn write_byte<D>(&mut self, addr: u32, byte: u8, disp: &mut D)
    where
        D: Display,
    {
        self.write_u8(addr, byte, disp);
    }
    pub fn read_byte(&mut self, addr: u32) -> u8 {
        let addr_u = addr as usize;
        self.check_access(addr_u, 1);

        let byte_1 = self.heap.get(addr_u).unwrap();

        *byte_1
    }
}
