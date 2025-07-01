pub fn nand16(a: u16, b: u16) -> u16 {
    !(a & b)
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
}
