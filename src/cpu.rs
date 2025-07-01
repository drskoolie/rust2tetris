use crate::alu::{ alu, AluFlags };
use crate::sequential::{Register16, Counter16};

pub struct Cpu {
    pub a: Register16,
    pub d: Register16,
    pub pc: Counter16,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: Register16::new(),
            d: Register16::new(),
            pc: Counter16::new(),
        }
    }

    pub fn get_a(&self) -> u16 {
        self.a.get_output()
    }

    pub fn get_d(&self) -> u16 {
        self.d.get_output()
    }

    pub fn get_pc(&self) -> u16 {
        self.pc.get_output()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_new() {
        let cpu = Cpu::new();

        assert_eq!(cpu.get_a(), 0);
        assert_eq!(cpu.get_d(), 0);
        assert_eq!(cpu.get_pc(), 0);

    }
}
