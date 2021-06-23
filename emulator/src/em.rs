use crate::{
    asm,
    target::{CLIDebugger, EmptyDebugger},
    traits::Debugger,
    Display, Exception, Input, Memory, CODE_MAPPING_OFFSET, CPU,
};

/// The actual Emulator itself
pub struct Emulator<K, D>
where
    K: Input,
    D: Display,
{
    file: Option<g3a::File>,
    cpu: CPU,
    memory: Memory,
    input: K,
    display: D,
    debugger: Box<dyn Debugger>,
}

impl<K, D> Emulator<K, D>
where
    K: Input,
    D: Display,
{
    pub fn new(file: g3a::File, input: K, display: D) -> Self {
        let mut memory = Memory::new_with_code_size(file.executable_code.len());
        memory.write_register(15, 0x80000);
        memory.write_register(14, 0x80000);

        for (i, tmp) in file.executable_code.iter().enumerate() {
            memory.write_byte(i as u32 + CODE_MAPPING_OFFSET, *tmp);
        }

        Self {
            file: Some(file),
            cpu: CPU::new(CODE_MAPPING_OFFSET),
            memory,
            input,
            display,
            debugger: Box::new(EmptyDebugger::new()),
        }
    }

    pub fn new_test(input: K, display: D, instructions: Vec<asm::Instruction>) -> Self {
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
            cpu: CPU::new(CODE_MAPPING_OFFSET),
            memory,
            input,
            display,
            debugger: Box::new(CLIDebugger::new()),
        }
    }
    pub fn new_test_raw(input: K, display: D, instr: Vec<u8>, mut memory: Memory) -> Self {
        for (index, byte) in instr.iter().enumerate() {
            memory.write_byte(index as u32 + CODE_MAPPING_OFFSET, *byte);
        }

        Self {
            file: None,
            cpu: CPU::new(CODE_MAPPING_OFFSET),
            memory,
            input,
            display,
            debugger: Box::new(CLIDebugger::new()),
        }
    }

    /// Changes the Debugger used by the Emulator
    pub fn set_debug(&mut self, p_debugger: Box<dyn Debugger>) {
        self.debugger = p_debugger;
    }

    pub fn pc(&self) -> u32 {
        self.cpu.pc()
    }

    pub fn clone_registers(&self) -> [u32; 16] {
        self.memory.clone_registers()
    }
    pub fn clone_heap(&self) -> Vec<u8> {
        self.memory.clone_heap()
    }

    /// Prints the Code starting
    ///
    /// Params:
    /// * p_start: The starting Offset from the CODE_MAPPING_OFFSET
    /// * p_length: How many instructions to print (each instruction is 2 bytes)
    pub fn print_code(&mut self, p_start: Option<usize>, p_length: Option<usize>) {
        let start = p_start.unwrap_or(0);
        let length = p_length.unwrap_or_else(|| {
            let file = self.file.as_ref().unwrap();
            file.executable_code.len() - 1
        });

        for offset in start..length {
            if offset % 2 != 0 {
                continue;
            }
            let index = start + offset + CODE_MAPPING_OFFSET as usize;
            let instr = self.fetch_instruction(index as u32);

            self.debugger.print_instr(index as u32, &instr);
        }
    }

    /// Prints the content of the Registers
    pub fn print_registers(&self) {
        println!(
            "PC: x{:X} PR: x{:X} T: {}",
            self.cpu.pc(),
            self.memory.pr,
            self.memory.t
        );
        self.memory.print_registers();
    }

    /// Prints out all the Data on the current Stack
    pub fn print_stack(&self) {
        let start = self.memory.read_register(15) + 1;
        let end = self.memory.read_register(14);

        self.memory.print_memory(start as usize, end as usize);
    }

    pub fn get_instr(&mut self, offset: u32) -> asm::Instruction {
        self.fetch_instruction(self.cpu.pc() + offset)
    }

    pub fn fetch_instruction(&mut self, pc: u32) -> asm::Instruction {
        let word_bytes = self.memory.read_word(pc);
        asm::Instruction::parse(word_bytes)
    }

    /// Runs until the given PC has been reached or an exception was raised
    pub async fn run_until(&mut self, target_pc: u32) -> Result<(), Exception> {
        loop {
            self.emulate_single().await?;
            if self.pc() >= target_pc {
                return Ok(());
            }
        }
    }
    /// Runs until the Program returned or an exception was raised
    pub async fn run_completion(&mut self) -> Result<(), Exception> {
        loop {
            self.emulate_single().await?;

            if self.cpu.pc() == 0 {
                return Ok(());
            }
        }
    }

    /// Emulates the execution of a single Instruction on the CPU
    pub async fn emulate_single(&mut self) -> Result<(), Exception> {
        self.cpu
            .tick(
                &mut self.memory,
                &mut self.display,
                &mut self.input,
                self.debugger.as_ref(),
            )
            .await
    }

    pub fn get_display_mut(&mut self) -> &mut D {
        &mut self.display
    }
    pub fn get_input_mut(&mut self) -> &mut K {
        &mut self.input
    }

    pub fn force_display(&mut self) {
        self.display.display_vram(&mut self.memory);
    }
}
