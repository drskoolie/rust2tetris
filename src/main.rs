mod hardware;
mod parser;

use crate::hardware::cpu::Cpu;
use crate::parser::assembly::Assembler;

fn main() {
    let mut cpu = Cpu::new();
    let mut asm = Assembler::new();
    let source = r#"
        @i
        M=1
        (LOOP)
        @i
        D=M
        @100
        D=D-A
        @END
        D;JGT
        @LOOP
        0;JMP
        (END)
        @END
        0;JMP
    "#;
    asm.assemble_all(source);

    cpu.reset_pc();
    cpu.load_from_string(&asm.binaries.join("\n"));
    cpu.run();
}
