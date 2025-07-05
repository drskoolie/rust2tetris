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

    stack.push_value(10);
    asm.assemble_all(&stack.assembly.join("\n"));

    cpu.reset_pc();
    cpu.load_from_string(&asm.binaries.join("\n"));
    cpu.run();
}
