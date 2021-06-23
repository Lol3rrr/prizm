#[cfg(feature = "wasm")]
use crate::wasm::log;

use crate::CODE_MAPPING_OFFSET;

/// The Offset at which the VRAM starts
pub const VRAM: u32 = 0xAC000000;
pub const HEAP_START: u32 = 0x881E0000;
pub const HEAP_END: u32 = 0x881FFFFF;
pub const HEAP_SIZE: u32 = HEAP_END - HEAP_START;
pub const VIRT_STACK_START: u32 = 0x08100000;

const DISPLAY_WIDTH: usize = 384;
const DISPLAY_HEIGHT: usize = 216;

pub struct Memory {
    registers: [u32; 16],
    pub pr: u32,
    pub t: bool,
    pub macl: u32,
    pub mach: u32,
    heap: Vec<u8>,
    vram: [u8; DISPLAY_HEIGHT * DISPLAY_WIDTH * 2],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
            pr: 0,
            t: false,
            macl: 0,
            mach: 0,
            heap: Vec::with_capacity(CODE_MAPPING_OFFSET as usize),
            vram: [0; DISPLAY_WIDTH * DISPLAY_HEIGHT * 2],
        }
    }
    pub fn new_with_code_size(size: usize) -> Self {
        let heap_size = (CODE_MAPPING_OFFSET as usize) + size;

        Self {
            registers: [0; 16],
            pr: 0,
            t: false,
            macl: 0,
            mach: 0,
            heap: vec![0; heap_size],
            vram: [0; DISPLAY_WIDTH * DISPLAY_HEIGHT * 2],
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

    pub fn get_vram(&self, addr: usize) -> u8 {
        self.vram[addr]
    }

    fn write_u8(&mut self, addr: u32, value: u8) {
        if addr >= VRAM {
            let vram_addr = addr - VRAM;
            self.vram[vram_addr as usize] = value;
            return;
        }
        let addr_u = addr as usize;

        self.heap[addr_u] = value;
    }
    fn read_u8(&mut self, addr: u32) -> u8 {
        if addr >= VRAM {
            let vram_addr = addr - VRAM;
            return self.vram[vram_addr as usize];
        }

        let addr_u = addr as usize;
        if addr_u >= self.heap.len() {
            return 0;
        }

        return self.heap[addr_u];
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

    fn check_access(&mut self, addr: u32, size: usize) {
        if addr >= VRAM {
            return;
        }

        let addr_u = addr as usize;
        let expected_size = addr_u + size + 1;
        if self.heap.len() < expected_size {
            println!("Resizing to {:x}", expected_size);
            self.heap.resize(expected_size, 0);
        }
    }
    pub fn write_long(&mut self, addr: u32, value: u32) {
        if addr & 0x3 > 0 {
            panic!("Unaligned Long-Write to Address: x{:X}", addr);
        }

        self.check_access(addr, 4);

        let bytes = value.to_be_bytes();
        self.write_u8(addr, bytes[0]);
        self.write_u8(addr + 1, bytes[1]);
        self.write_u8(addr + 2, bytes[2]);
        self.write_u8(addr + 3, bytes[3]);
    }
    pub fn read_long(&mut self, addr: u32) -> u32 {
        let byte_1 = self.read_u8(addr);
        let byte_2 = self.read_u8(addr + 1);
        let byte_3 = self.read_u8(addr + 2);
        let byte_4 = self.read_u8(addr + 3);

        let result = u32::from_be_bytes([byte_1, byte_2, byte_3, byte_4]);

        result
    }

    pub fn write_word(&mut self, addr: u32, value: u16) {
        if addr & 0x1 > 0 {
            panic!("Unaligned Word-Write to Address: x{:X}", addr);
        }

        self.check_access(addr, 2);

        let bytes = value.to_be_bytes();
        self.write_u8(addr, bytes[0]);
        self.write_u8(addr + 1, bytes[1]);
    }
    pub fn read_word(&mut self, addr: u32) -> u16 {
        let byte_1 = self.read_u8(addr);
        let byte_2 = self.read_u8(addr + 1);

        u16::from_be_bytes([byte_1, byte_2])
    }

    pub fn write_byte(&mut self, addr: u32, byte: u8) {
        self.check_access(addr, 1);

        self.write_u8(addr, byte);
    }
    pub fn read_byte(&mut self, addr: u32) -> u8 {
        self.read_u8(addr)
    }

    pub fn set_vram(&mut self, data: &[u8]) {
        self.vram.copy_from_slice(data);
    }
}
