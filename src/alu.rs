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

    #[test]
    fn test_alu_zx_false() {
        assert_eq!(
            alu(
                0x0001, // x
                0x0, // y
                false, // zx
                false, // nx
                false, // zy
                false, // ny
                true, // f
                false // no
                ),
            (0x0001, false, false));
    }

    #[test]
    fn test_alu_zy_true() {
        assert_eq!(
            alu(
                0x0000, // x
                0x0001, // y
                false, // zx
                false, // nx
                true, // zy
                false, // ny
                true, // f
                false // no
                ),
            (0x0000, true, false));
    }

    #[test]
    fn test_alu_zy_false() {
        assert_eq!(
            alu(
                0x0000, // x
                0x0001, // y
                false, // zx
                false, // nx
                false, // zy
                false, // ny
                true, // f
                false // no
                ),
            (0x0001, false, false));
    }
    
    #[test]
    fn test_alu_nx_true() {
        assert_eq!(
            alu(
                0x0001, // x
                0x0000, // y
                false, // zx
                true, // nx
                false, // zy
                false, // ny
                true, // f
                false // no
                ),
            (0xFFFE, false, true));
    }
    
    #[test]
    fn test_alu_nx_false() {
        assert_eq!(
            alu(
                0x0001, // x
                0x0000, // y
                false, // zx
                false, // nx
                false, // zy
                false, // ny
                true, // f
                false // no
                ),
            (0x0001, false, false));
    }

    #[test]
    fn test_alu_ny_true() {
        assert_eq!(
            alu(
                0x0000, // x
                0x0001, // y
                false, // zx
                false, // nx
                false, // zy
                true, // ny
                true, // f
                false // no
                ),
            (0xFFFE, false, true));
    }

    #[test]
    fn test_alu_ny_false() {
        assert_eq!(
            alu(
                0x0000, // x
                0x0001, // y
                false, // zx
                false, // nx
                false, // zy
                false, // ny
                true, // f
                false // no
                ),
            (0x0001, false, false));
    }

    #[test]
    fn test_alu_f_true() {
        assert_eq!(
            alu(
                0x0001, // x
                0x0001, // y
                false, // zx
                false, // nx
                false, // zy
                false, // ny
                true, // f
                false // no
                ),
            (0x0002, false, false));
    }

    #[test]
    fn test_alu_f_false() {
        assert_eq!(
            alu(
                0b0011, // x
                0b0101, // y
                false, // zx
                false, // nx
                false, // zy
                false, // ny
                false, // f
                false // no
                ),
            (0b0001, false, false));
    }

    #[test]
    fn test_alu_no_true() {
        assert_eq!(
            alu(
                0b0001, // x
                0b0000, // y
                false, // zx
                false, // nx
                false, // zy
                false, // ny
                true, // f
                true // no
                ),
            (0xFFFE, false, true));
    }

    #[test]
    fn test_alu_no_false() {
        assert_eq!(
            alu(
                0b0001, // x
                0b0000, // y
                false, // zx
                false, // nx
                false, // zy
                false, // ny
                true, // f
                false // no
                ),
            (0x0001, false, false));
    }

    #[test]
    fn test_alu_zr_true() {
        assert_eq!(
            alu(
                0b0000, // x
                0b0000, // y
                false, // zx
                false, // nx
                false, // zy
                false, // ny
                true, // f
                false // no
                ),
            (0x0000, true, false));
    }

    #[test]
    fn test_alu_zr_false() {
        assert_eq!(
            alu(
                0b0001, // x
                0b0000, // y
                false, // zx
                false, // nx
                false, // zy
                false, // ny
                true, // f
                false // no
                ),
            (0x0001, false, false));
    }

    #[test]
    fn test_alu_zn_true() {
        assert_eq!(
            alu(
                0x8000, // x
                0x0000, // y
                false, // zx
                false, // nx
                false, // zy
                false, // ny
                true, // f
                false // no
                ),
            (0x8000, false, true));
    }

    #[test]
    fn test_alu_zn_false() {
        assert_eq!(
            alu(
                0x0001, // x
                0x0000, // y
                false, // zx
                false, // nx
                false, // zy
                false, // ny
                true, // f
                false // no
                ),
            (0x0001, false, false));
    }

    #[test]
    fn test_alu_out_0() {
        assert_eq!(
            alu(
                0x0001, // x
                0x0F00, // y
                true, // zx
                false, // nx
                true, // zy
                false, // ny
                true, // f
                false // no
                ),
            (0x0000, true, false));
    }

    #[test]
    fn test_alu_out_1() {
        assert_eq!(
            alu(
                0x0001, // x
                0x0F00, // y
                true, // zx
                true, // nx
                true, // zy
                true, // ny
                true, // f
                true // no
                ),
            (0x0001, false, false));
    }

    #[test]
    fn test_alu_out_neg_1() {
        assert_eq!(
            alu(
                0x0001, // x
                0x0F00, // y
                true, // zx
                true, // nx
                true, // zy
                false, // ny
                true, // f
                false // no
                ),
            (0xFFFF, false, true));
    }

    #[test]
    fn test_alu_out_x() {
        let x = 0x0F1E;
        assert_eq!(
            alu(
                x, // x
                0x01FA, // y
                false, // zx
                false, // nx
                true, // zy
                false, // ny
                true, // f
                false // no
                ),
            (x, false, false));
    }

    #[test]
    fn test_alu_out_y() {
        let y = 0x0F1E;
        assert_eq!(
            alu(
                0x014A, // x
                y, // y
                true, // zx
                false, // nx
                false, // zy
                false, // ny
                true, // f
                false // no
                ),
            (y, false, false));
    }

    #[test]
    fn test_alu_out_not_x() {
        let x = 0x0F1E;
        assert_eq!(
            alu(
                x, // x
                0x01FA, // y
                false, // zx
                false, // nx
                true, // zy
                true, // ny
                false, // f
                true // no
                ),
            (not16(x), false, true));
    }

    #[test]
    fn test_alu_out_not_y() {
        let y = 0x0F1E;
        assert_eq!(
            alu(
                0x014A, // x
                y, // y
                true, // zx
                true, // nx
                false, // zy
                false, // ny
                false, // f
                true // no
                ),
            (not16(y), false, true));
    }

    #[test]
    fn test_alu_out_neg_x() {
        let x = 0x0001;
        assert_eq!(
            alu(
                x, // x
                0x01FA, // y
                false, // zx
                false, // nx
                true, // zy
                true, // ny
                true, // f
                true // no
                ),
            (0xFFFF, false, true));
    }

    #[test]
    fn test_alu_out_neg_y() {
        let y = 0x0001;
        assert_eq!(
            alu(
                0x014A, // x
                y, // y
                true, // zx
                true, // nx
                false, // zy
                false, // ny
                true, // f
                true // no
                ),
            (0xFFFF, false, true));
    }


}
