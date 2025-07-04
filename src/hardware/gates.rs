pub fn get_bit(a: u16, i: usize) -> bool {
    assert!(i < 16);
    (a >> i) & 1 != 0
}

pub fn set_bit(a: u16, i: usize, value: bool) -> u16 {
    assert!(i < 16);
    if value {
        a | (1 << i)
    } else {
        a & !(1 << i)
    }
}

pub fn nand16(a: u16, b: u16) -> u16 {
    !(a & b)
}

pub fn not16(a: u16) -> u16 {
    nand16(a, a)
}

pub fn and16(a: u16, b: u16) -> u16 {
    not16(nand16(a, b))
}

pub fn half_adder(a: bool, b: bool) -> (bool, bool) {
    let sum = a ^ b;
    let carry = a & b;
    (sum, carry)
}

pub fn full_adder(a: bool, b: bool, carry_in: bool) -> (bool, bool) {
    let (sum1, carry1) = half_adder(a, b);
    let (sum2, carry2) = half_adder(sum1, carry_in);
    let carry_out = carry1 || carry2;
    (sum2, carry_out)
}

pub fn add16(a: u16, b:u16) -> u16 {
    let mut carry: bool = false;

    let mut result: u16 = 0x0000;

    for i in 0..16 {
        let a_bit = get_bit(a, i);
        let b_bit = get_bit(b, i);

        let (sum, carry_next) = full_adder(a_bit, b_bit, carry);
        carry = carry_next;

        if sum {
            result |= set_bit(0x0, i, true);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bit() {
        let value: u16 = 0b1010_0000_0000_1010;

        assert!(!get_bit(value, 0));
        assert!(get_bit(value, 1));
        assert!(!get_bit(value, 2));
        assert!(get_bit(value, 3));
        assert!(!get_bit(value, 4));
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_get_bit_invalid_index() {
        get_bit(0xFFFF, 16);
    }

    #[test]
    fn test_set_bit() {
        let a: u16 = 0x0000;
        let b: u16 = 0xFFFF;

        let c = set_bit(a, 0, true);
        assert_eq!(c, 0b1);

        let c = set_bit(a, 1, true);
        assert_eq!(c, 0b10);

        let c = set_bit(b, 0, false);
        assert_eq!(c, 0xFFFE);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_set_bit_invalid_index() {
        let a: u16 = 0xFFFF;
        set_bit(a, 16, true);
    }

    #[test]
    fn test_nand16() {
        assert_eq!(nand16(0xFFFF, 0xFFFF), 0x0000);
        assert_eq!(nand16(0x0000, 0x0000), 0xFFFF);
        assert_eq!(nand16(0xFFFF, 0x0000), 0xFFFF);
    }

    #[test]
    fn test_not16() {
        assert_eq!(not16(0x0000), 0xFFFF);
        assert_eq!(not16(0xFFFF), 0x0000);
    }

    #[test]
    fn test_and16() {
        assert_eq!(and16(0b0011, 0b0101), 0b0001);
    }

    #[test]
    fn test_half_adder() {
        assert_eq!(half_adder(false, false), (false, false));
        assert_eq!(half_adder(false, true), (true, false));
        assert_eq!(half_adder(true, false), (true, false));
        assert_eq!(half_adder(true, true), (false, true));
    }

    #[test]
    fn test_full_adder() {
        assert_eq!(full_adder(false, false, false), (false, false));
        assert_eq!(full_adder(false, false, true), (true, false));
        assert_eq!(full_adder(false, true, false), (true, false));
        assert_eq!(full_adder(false, true, true), (false, true));
        assert_eq!(full_adder(true, false, false), (true, false));
        assert_eq!(full_adder(true, false, true), (false, true));
        assert_eq!(full_adder(true, true, false), (false, true));
        assert_eq!(full_adder(true, true, true), (true, true));
    }

    #[test]
    fn test_add16() {
        assert_eq!(add16(0b0001, 0b0001), 0b0010);
        assert_eq!(add16(0b0101, 0b0000), 0b0101);
    }

}
