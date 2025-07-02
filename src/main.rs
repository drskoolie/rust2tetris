mod alu;
mod cpu;
mod gates;
mod memory;

use cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();
    cpu.reset_pc();
    cpu.load_from_file("program.hack");
    cpu.run();
}
