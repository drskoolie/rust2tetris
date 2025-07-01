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
    cpu.set_d(0x00FF);
    cpu.set_pc(0xFFFF, false, true, false);
    cpu.execute(0x7FFF);
    cpu.tick();
    cpu.print_cpu();
}
