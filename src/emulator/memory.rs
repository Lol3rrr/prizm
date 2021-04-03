pub struct Memory {
    registers: [u32; 16],
    pub pr: u32,
    heap: Vec<u32>,
}
impl Memory {
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
            pr: 0,
            heap: Vec::new(),
        }
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

    pub fn write_heap(&mut self, addr: u32, value: u32) {
        match self.heap.get_mut(addr as usize) {
            Some(mem) => {
                *mem = value;
            }
            None => {
                self.heap.resize((addr + 1) as usize, 0);
                *self.heap.get_mut(addr as usize).unwrap() = value;
            }
        };
    }
    pub fn read_heap(&self, addr: u32) -> u32 {
        match self.heap.get(addr as usize) {
            Some(mem) => *mem,
            None => 0,
        }
    }
}
