use crate::gates::{
    not16,
    and16,
    add16,
};

pub struct AluFlags {
    pub zx: bool, // Zero the x input
    pub nx: bool, // Negate the x input
    pub zy: bool, // Zero the y input
    pub ny: bool, // Negate the y input
    pub f: bool, // true: add, false: and
    pub no: bool // Negate the output
}

pub fn alu(
    mut x: u16,
    mut y: u16,
    flags: AluFlags,
    ) -> (
    u16, // output
    bool, // True iff out=0
    bool // True iff out <0
    ) {

    if flags.zx {
        x = and16(x, 0x0000);
    }

    if flags.nx {
        x = not16(x);
    }

    if flags.zy {
        y = and16(y, 0x0000);
    }

    if flags.ny {
        y = not16(y);
    }

    let mut out: u16;

    if flags.f {
        out = add16(x, y);
    } else {
        out = and16(x, y);
    }

    if flags.no {
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
        let flags = AluFlags {
            zx: true, // zx
            nx: false, // nx
            zy: false, // zy
            ny: false, // ny
            f: true, // f
            no: false // no
        };

        assert_eq!(
            alu(
                0x0001, // x
                0x0, // y
                flags,
                ),
            (0x0000, true, false));
    }

    #[test]
    fn test_alu_zx_false() {
        let flags = AluFlags {
            zx: false, // zx
            nx: false, // nx
            zy: false, // zy
            ny: false, // ny
            f: true, // f
            no: false // no
        };

        assert_eq!(
            alu(
                0x0001, // x
                0x0, // y
                flags,
                ),
            (0x0001, false, false));
    }

    #[test]
    fn test_alu_zy_true() {
        let flags = AluFlags {
            zx: false, // zx
            nx: false, // nx
            zy: true, // zy
            ny: false, // ny
            f: true, // f
            no: false // no
        };

        assert_eq!(
            alu(
                0x0000, // x
                0x0001, // y
                flags,
                ),
            (0x0000, true, false));
    }

    #[test]
    fn test_alu_zy_false() {
        let flags = AluFlags {
            zx: false, // zx
            nx: false, // nx
            zy: false, // zy
            ny: false, // ny
            f: true, // f
            no: false // no
        };

        assert_eq!(
            alu(
                0x0000, // x
                0x0001, // y
                flags,
                ),
            (0x0001, false, false));
    }
    
    #[test]
    fn test_alu_nx_true() {
        let flags = AluFlags {
            zx: false, // zx
            nx: true, // nx
            zy: false, // zy
            ny: false, // ny
            f: true, // f
            no: false // no
        };

        assert_eq!(
            alu(
                0x0001, // x
                0x0000, // y
                flags,
                ),
            (0xFFFE, false, true));
    }
    
    #[test]
    fn test_alu_nx_false() {
        let flags = AluFlags {
            zx: false, // zx
            nx: false, // nx
            zy: false, // zy
            ny: false, // ny
            f: true, // f
            no: false // no
        };

        assert_eq!(
            alu(
                0x0001, // x
                0x0000, // y
                flags,
                ),
            (0x0001, false, false));
    }

    #[test]
    fn test_alu_ny_true() {
        let flags = AluFlags {
            zx: false, // zx
            nx: false, // nx
            zy: false, // zy
            ny: true, // ny
            f: true, // f
            no: false // no
        };

        assert_eq!(
            alu(
                0x0000, // x
                0x0001, // y
                flags,
                ),
            (0xFFFE, false, true));
    }

    #[test]
    fn test_alu_ny_false() {
        let flags = AluFlags {
            zx: false, // zx
            nx: false, // nx
            zy: false, // zy
            ny: false, // ny
            f: true, // f
            no: false // no
        };

        assert_eq!(
            alu(
                0x0000, // x
                0x0001, // y
                flags,
                ),
            (0x0001, false, false));
    }

    #[test]
    fn test_alu_f_true() {
        let flags = AluFlags {
            zx: false, // zx
            nx: false, // nx
            zy: false, // zy
            ny: false, // ny
            f: true, // f
            no: false // no
        };

        assert_eq!(
            alu(
                0x0001, // x
                0x0001, // y
                flags,
                ),
            (0x0002, false, false));
    }

    #[test]
    fn test_alu_f_false() {
        let flags = AluFlags {
            zx: false, // zx
            nx: false, // nx
            zy: false, // zy
            ny: false, // ny
            f: false, // f
            no: false // no
        };

        assert_eq!(
            alu(
                0b0011, // x
                0b0101, // y
                flags,
                ),
            (0b0001, false, false));
    }

    #[test]
    fn test_alu_no_true() {
        let flags = AluFlags {
            zx: false, // zx
            nx: false, // nx
            zy: false, // zy
            ny: false, // ny
            f: true, // f
            no: true // no
        };

        assert_eq!(
            alu(
                0b0001, // x
                0b0000, // y
                flags,
                ),
            (0xFFFE, false, true));
    }

    #[test]
    fn test_alu_no_false() {
        let flags = AluFlags {
            zx: false, // zx
            nx: false, // nx
            zy: false, // zy
            ny: false, // ny
            f: true, // f
            no: false // no
        };

        assert_eq!(
            alu(
                0b0001, // x
                0b0000, // y
                flags,
                ),
            (0x0001, false, false));
    }

    #[test]
    fn test_alu_zr_true() {
        let flags = AluFlags {
            zx: false, // zx
            nx: false, // nx
            zy: false, // zy
            ny: false, // ny
            f: true, // f
            no: false // no
        };

        assert_eq!(
            alu(
                0b0000, // x
                0b0000, // y
                flags,
                ),
            (0x0000, true, false));
    }

    #[test]
    fn test_alu_zr_false() {
        let flags = AluFlags {
            zx: false, // zx
            nx: false, // nx
            zy: false, // zy
            ny: false, // ny
            f: true, // f
            no: false // no
        };

        assert_eq!(
            alu(
                0b0001, // x
                0b0000, // y
                flags,
                ),
            (0x0001, false, false));
    }

    #[test]
    fn test_alu_zn_true() {
        let flags = AluFlags {
            zx: false, // zx
            nx: false, // nx
            zy: false, // zy
            ny: false, // ny
            f: true, // f
            no: false // no
        };

        assert_eq!(
            alu(
                0x8000, // x
                0x0000, // y
                flags,
                ),
            (0x8000, false, true));
    }

    #[test]
    fn test_alu_zn_false() {
        let flags = AluFlags {
            zx: false, // zx
            nx: false, // nx
            zy: false, // zy
            ny: false, // ny
            f: true, // f
            no: false // no
        };

        assert_eq!(
            alu(
                0x0001, // x
                0x0000, // y
                flags,
                ),
            (0x0001, false, false));
    }

    #[test]
    fn test_alu_out_0() {
        let flags = AluFlags {
            zx: true, // zx
            nx: false, // nx
            zy: true, // zy
            ny: false, // ny
            f: true, // f
            no: false // no
        };

        assert_eq!(
            alu(
                0x0001, // x
                0x0F00, // y
                flags,
                ),
            (0x0000, true, false));
    }

    #[test]
    fn test_alu_out_1() {
        let flags = AluFlags {
            zx: true, // zx
            nx: true, // nx
            zy: true, // zy
            ny: true, // ny
            f: true, // f
            no: true // no
        };

        assert_eq!(
            alu(
                0x0001, // x
                0x0F00, // y
                flags,
                ),
            (0x0001, false, false));
    }

    #[test]
    fn test_alu_out_neg_1() {
        let flags = AluFlags {
            zx: true, // zx
            nx: true, // nx
            zy: true, // zy
            ny: false, // ny
            f: true, // f
            no: false // no
        };

        assert_eq!(
            alu(
                0x0001, // x
                0x0F00, // y
                flags,
                ),
            (0xFFFF, false, true));
    }

    #[test]
    fn test_alu_out_x() {
        let x = 0x0F1E;
        let flags = AluFlags {
            zx: false, // zx
            nx: false, // nx
            zy: true, // zy
            ny: false, // ny
            f: true, // f
            no: false // no
        };

        assert_eq!(
            alu(
                x, // x
                0x01FA, // y
                flags,
                ),
            (x, false, false));
    }

    #[test]
    fn test_alu_out_y() {
        let flags = AluFlags {
            zx: true, // zx
            nx: false, // nx
            zy: false, // zy
            ny: false, // ny
            f: true, // f
            no: false // no
        };

        let y = 0x0F1E;
        assert_eq!(
            alu(
                0x014A, // x
                y, // y
            flags,
                ),
            (y, false, false));
    }

    #[test]
    fn test_alu_out_not_x() {
        let flags = AluFlags {
            zx: false, // zx
            nx: false, // nx
            zy: true, // zy
            ny: true, // ny
            f: false, // f
            no: true // no
        };

        let x = 0x0F1E;
        assert_eq!(
            alu(
                x, // x
                0x01FA, // y
                flags,
                ),
            (not16(x), false, true));
    }

    #[test]
    fn test_alu_out_not_y() {
        let flags = AluFlags {
            zx: true, // zx
            nx: true, // nx
            zy: false, // zy
            ny: false, // ny
            f: false, // f
            no: true // no
        };

        let y = 0x0F1E;
        assert_eq!(
            alu(
                0x014A, // x
                y, // y
            flags,
                ),
            (not16(y), false, true));
    }

    #[test]
    fn test_alu_out_neg_x() {
        let flags = AluFlags {
            zx: false, // zx
            nx: false, // nx
            zy: true, // zy
            ny: true, // ny
            f: true, // f
            no: true // no
        };

        let x = 0x0001;
        assert_eq!(
            alu(
                x, // x
                0x01FA, // y
                flags,
                ),
            (0xFFFF, false, true));
    }

    #[test]
    fn test_alu_out_neg_y() {
        let flags = AluFlags {
            zx: true, // zx
            nx: true, // nx
            zy: false, // zy
            ny: false, // ny
            f: true, // f
            no: true // no
        };

        let y = 0x0001;
        assert_eq!(
            alu(
                0x014A, // x
                y, // y
            flags,
                ),
            (0xFFFF, false, true));
    }

    #[test]
    fn test_alu_out_inc_x() {
        let flags = AluFlags {
            zx: false, // zx
            nx: true, // nx
            zy: true, // zy
            ny: true, // ny
            f: true, // f
            no: true // no
        };

        let x = 0x0001;
        assert_eq!(
            alu(
                x, // x
                0x01FA, // y
                flags,
                ),
            (0x0002, false, false));
    }

    #[test]
    fn test_alu_out_inc_y() {
        let flags = AluFlags {
            zx: true, // zx
            nx: true, // nx
            zy: false, // zy
            ny: true, // ny
            f: true, // f
            no: true // no
        };

        let y = 0x0001;
        assert_eq!(
            alu(
                0x014A, // x
                y, // y
            flags,
                ),
            (0x0002, false, false));
    }

    #[test]
    fn test_alu_out_dec_x() {
        let flags = AluFlags {
            zx: false, // zx
            nx: false, // nx
            zy: true, // zy
            ny: true, // ny
            f: true, // f
            no: false // no
        };

        let x = 0x0002;
        assert_eq!(
            alu(
                x, // x
                0x01FA, // y
                flags,
                ),
            (0x0001, false, false));
    }

    #[test]
    fn test_alu_out_dec_y() {
        let flags = AluFlags {
            zx: true, // zx
            nx: true, // nx
            zy: false, // zy
            ny: false, // ny
            f: true, // f
            no: false // no
        };

        let y = 0x0002;
        assert_eq!(
            alu(
                0x014A, // x
                y, // y
            flags,
                ),
            (0x0001, false, false));
    }

    #[test]
    fn test_alu_out_x_plus_y() {
        let flags = AluFlags {
            zx: false, // zx
            nx: false, // nx
            zy: false, // zy
            ny: false, // ny
            f: true, // f
            no: false // no
        };

        let x = 0x0002;
        let y = 0x0003;
        assert_eq!(
            alu(
                x, // x
                y, // y
            flags,
                ),
            (x+y, false, false));
    }

    #[test]
    fn test_alu_out_x_minus_y() {
        let flags = AluFlags {
            zx: false, // zx
            nx: true, // nx
            zy: false, // zy
            ny: false, // ny
            f: true, // f
            no: true // no
        };

        let x = 0x0002;
        let y = 0x0003;
        assert_eq!(
            alu(
                x, // x
                y, // y
            flags,
                ),
            (x.wrapping_sub(y), false, true));
    }

    #[test]
    fn test_alu_out_y_minus_x() {
        let flags = AluFlags {
            zx: false, // zx
            nx: false, // nx
            zy: false, // zy
            ny: true, // ny
            f: true, // f
            no: true // no
        };

        let x = 0x0002;
        let y = 0x0003;
        assert_eq!(
            alu(
                x, // x
                y, // y
            flags,
                ),
            (y.wrapping_sub(x), false, false));
    }

    #[test]
    fn test_alu_out_x_and_y() {
        let flags = AluFlags {
            zx: false, // zx
            nx: false, // nx
            zy: false, // zy
            ny: false, // ny
            f: false, // f
            no: false // no
        };

        let x = 0x0002;
        let y = 0x0003;
        assert_eq!(
            alu(
                x, // x
                y, // y
            flags,
                ),
            (x & y, false, false));
    }

    #[test]
    fn test_alu_out_x_or_y() {
        let flags = AluFlags {
            zx: false, // zx
            nx: true, // nx
            zy: false, // zy
            ny: true, // ny
            f: false, // f
            no: true // no
        };

        let x = 0x0002;
        let y = 0x0003;
        assert_eq!(
            alu(
                x, // x
                y, // y
            flags,
                ),
            (x | y, false, false));
    }
}
