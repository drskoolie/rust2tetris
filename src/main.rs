mod hardware;
mod parser;
mod vm;

use crate::hardware::cpu::Cpu;
use crate::parser::assembly::Assembler;
use crate::vm::vm::Stack;

fn main() {
    let mut cpu = Cpu::new();
    let mut asm = Assembler::new();
    let mut stack = Stack::new();

    stack.commands = vec![
        "push constant 7".into(),
    ];
    stack.assemble_all();
    asm.assemble_all(&stack.assembly.join("\n"));

    cpu.reset_pc();
    cpu.load_from_string(&asm.binaries.join("\n"));
    cpu.run_print();
    cpu.run();
}
