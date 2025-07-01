use crate::gates::{
    not16,
    and16,
    add16,
};

pub fn alu(
    mut x: u16,
    mut y: u16,
    zx: bool, // Zero the x input
    nx: bool, // Negate the x input
    zy: bool, // Zero the y input
    ny: bool, // Negate the y input
    f: bool, // true: add, false: and
    no: bool // Negate the output
    ) -> (
    u16, // output
    bool, // True iff out=0
    bool // True iff out <0
    ) {

    if zx {
        x = and16(x, 0x0000);
    }

    if nx {
        x = not16(x);
    }

    if zy {
        y = and16(y, 0x0000);
    }

    if ny {
        y = not16(y);
    }

    let mut out: u16;

    if f {
        out = add16(x, y);
    } else {
        out = and16(x, y);
    }

    if no {
        out = not16(out);
    }

    let zr = out == 0;
    let ng = (out & 0x8000) != 0;

    (out, zr, ng)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alu_zx_true() {
        assert_eq!(
            alu(
                0x0001, // x
                0x0, // y
                true, // zx
                false, // nx
                false, // zy
                false, // ny
                true, // f
                false // no
                ),
            (0x0000, true, false));
    }
    
}
