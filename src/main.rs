mod alu;
mod cpu;
mod gates;
mod sequential;

use alu::{ alu, AluFlags };
use cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();
    let flags_alu = AluFlags  {
        zx: false,
        nx: false,
        zy: false,
        ny: false,
        f: true,
        no: false
    };

    let (output, _zr, _ng) = alu(0x1, 0x1, flags_alu);

    println!("ALU: {:016b}", output);
    cpu.print_cpu();
    cpu.set_a(0xFF00, true);
    cpu.set_d(0x00FF, true);
    cpu.set_pc(0xFFFF, false, true, false);
    cpu.tick();
    cpu.print_cpu();
}
