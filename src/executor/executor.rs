use crate::hardware::cpu::Cpu;
use crate::parser::assembly::Assembler;
use crate::stack::stack::Stack;

pub struct Executor {
    cpu: Cpu,
    asm: Assembler,
    stack: Stack,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            asm: Assembler::new(),
            stack: Stack::new(),
        }
    }

    pub fn set_stack(&mut self, commands: Vec<String>) {
        self.stack.commands = commands;
    }

    pub fn assemble_all(&mut self) {
        self.stack.assemble_all();
        self.asm.assemble_all(&self.stack.assembly.join("\n"));

        self.cpu.reset_pc();
        self.cpu.load_from_string(&self.asm.binaries.join("\n"));
    }

    pub fn run(&mut self) {
        self.assemble_all();
        self.cpu.run();
    }

    pub fn run_print(&mut self) {
        self.assemble_all();
        self.cpu.run_print();
    }

}
