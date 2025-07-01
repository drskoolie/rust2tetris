mod alu;
mod gates;
mod sequential;

use sequential::{Register16, Counter16};
use alu::alu;

fn main() {
    let mut reg = Register16::new();
    let mut counter = Counter16::new();
    let (output, _zr, _ng) = alu(
        0x1,
        0x1,
        false,
        false,
        false,
        false,
        true,
        false);

    reg.set_input(0xAAAA, true);
    reg.tick();
    counter.set_input(0xFFFF, false, true, false);
    counter.tick();
    println!("Regeister: {:016b}", reg.get_output());
    println!("ALU: {:016b}", output);
    println!("Counter: {:016b}", counter.get_output());
}
