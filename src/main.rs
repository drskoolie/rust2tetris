mod alu;
mod cpu;
mod gates;
mod memory;
mod parser;

use cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();
    cpu.reset_pc();
    cpu.load_from_string("program.hack");
    cpu.run();
}
