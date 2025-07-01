mod alu;
mod cpu;
mod gates;
mod sequential;

use cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();
    cpu.print_cpu();
    cpu.set_d(0x00FF);
    cpu.set_pc(0xFFFF);
    cpu.execute(0x7FFF);
    cpu.tick();
    cpu.print_cpu();
}
