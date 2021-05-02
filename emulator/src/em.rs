use crate::{
    asm,
    target::{CLIDebugger, EmptyDebugger},
    traits::Debugger,
    Display, Exception, Input, Memory, CODE_MAPPING_OFFSET, CPU,
};

/// The actual Emulator itself
pub struct Emulator<'k, 'd, K, D>
where
    K: Input,
    D: Display,
{
    file: Option<g3a::File>,
    cpu: CPU,
    memory: Memory,
    input: &'k mut K,
    display: &'d mut D,
    debugger: Box<dyn Debugger>,
}

impl<'k, 'd, K, D> Emulator<'k, 'd, K, D>
where
    K: Input,
    D: Display,
{
    pub fn new<'b, 'c>(file: g3a::File, input: &'b mut K, display: &'c mut D) -> Self
    where
        'b: 'k,
        'c: 'd,
    {
        let mut memory = Memory::new();
        memory.write_register(15, 0x80000);
        memory.write_register(14, 0x80000);

        for (i, tmp) in file.executable_code.iter().enumerate() {
            memory.write_byte(i as u32 + CODE_MAPPING_OFFSET, *tmp, display);
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

    pub fn new_test<'b, 'c>(
        input: &'b mut K,
        display: &'c mut D,
        instructions: Vec<asm::Instruction>,
    ) -> Self
    where
        'b: 'k,
        'c: 'd,
    {
        let mut memory = Memory::new();
        memory.write_register(15, 0x80000);
        memory.write_register(14, 0x80000);

        let mut index = 0;
        for instr in instructions.iter() {
            let data = instr.to_byte();
            memory.write_byte(index + CODE_MAPPING_OFFSET, data[0], display);
            memory.write_byte(index + CODE_MAPPING_OFFSET + 1, data[1], display);

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
    pub fn new_test_raw<'b, 'c>(
        input: &'b mut K,
        display: &'c mut D,
        instr: Vec<u8>,
        mut memory: Memory,
    ) -> Self
    where
        'b: 'k,
        'c: 'd,
    {
        for (index, byte) in instr.iter().enumerate() {
            memory.write_byte(index as u32 + CODE_MAPPING_OFFSET, *byte, display);
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

    pub fn print_registers(&self) {
        println!(
            "PC: x{:X} PR: x{:X} T: {}",
            self.cpu.pc(),
            self.memory.pr,
            self.memory.t
        );
        self.memory.print_registers();
    }

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

            if self.cpu.pc() == 0 {
                return Ok(());
            }
        }
    }

    pub fn emulate_single(&mut self) -> Result<(), Exception> {
        self.cpu.tick(
            &mut self.memory,
            self.display,
            self.input,
            self.debugger.as_ref(),
        )
    }
}
