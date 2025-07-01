pub fn nand16(a: u16, b: u16) -> u16 {
    !(a & b)
}

pub fn not16(a: u16) -> u16 {
    nand16(a, a)
}

pub fn and16(a: u16, b: u16) -> u16 {
    not16(nand16(a, b))
}

pub fn or16(a: u16, b: u16) -> u16 {
    nand16(nand16(a, a), nand16(b, b))
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_or16() {
        assert_eq!(or16(0b0011, 0b0101), 0b0111);
    }
}
