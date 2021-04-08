const VRAM: u32 = 0xAC000000;

pub struct Memory {
    registers: [u32; 16],
    pub pr: u32,
    pub t: bool,
    heap: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
            pr: 0,
            t: false,
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

    fn write_u8(&mut self, addr: u32, value: u8) {
        if addr >= VRAM {
            println!("Writing to VRAM");
            return;
        }

        let addr_u = addr as usize;
        self.check_access(addr_u, 1);
        *self.heap.get_mut(addr_u).unwrap() = value;
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
    pub fn write_long(&mut self, addr: u32, value: u32) {
        let bytes = value.to_be_bytes();
        self.write_u8(addr, bytes[0]);
        self.write_u8(addr + 1, bytes[1]);
        self.write_u8(addr + 1, bytes[2]);
        self.write_u8(addr + 1, bytes[3]);
    }
    pub fn read_long(&mut self, addr: u32) -> u32 {
        let addr_u = addr as usize;
        self.check_access(addr_u, 4);

        let byte_1 = self.heap.get(addr_u).unwrap();
        let byte_2 = self.heap.get(addr_u + 1).unwrap();
        let byte_3 = self.heap.get(addr_u + 2).unwrap();
        let byte_4 = self.heap.get(addr_u + 3).unwrap();

        ((*byte_1 as u32) << 24)
            | ((*byte_2 as u32) << 16)
            | ((*byte_3 as u32) << 8)
            | (*byte_4 as u32)
    }

    pub fn read_word(&mut self, addr: u32) -> u16 {
        let addr_u = addr as usize;
        self.check_access(addr_u, 2);

        let byte_1 = self.heap.get(addr_u).unwrap();
        let byte_2 = self.heap.get(addr_u + 1).unwrap();

        ((*byte_1 as u16) << 8) | (*byte_2 as u16)
    }

    pub fn write_byte(&mut self, addr: u32, byte: u8) {
        self.write_u8(addr, byte);
    }
}
