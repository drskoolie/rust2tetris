mod alu;
mod gates;
mod sequential;

use sequential::Register;
use alu::alu;

fn main() {
    let mut reg = Register::new();
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
    println!("Output: {:016b}", reg.get_output());
    println!("Output: {:016b}", output);
}
