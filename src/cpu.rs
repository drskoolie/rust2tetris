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

    pub fn set_a(&mut self, value: u16, load: bool) {
        self.a.set_input(value, load);
    }

    pub fn get_d(&self) -> u16 {
        self.d.get_output()
    }

    pub fn set_d(&mut self, value: u16, load: bool) {
        self.d.set_input(value, load);
    }

    pub fn get_pc(&self) -> u16 {
        self.pc.get_output()
    }

    pub fn set_pc(&mut self, input: u16, reset: bool, load: bool, inc: bool) {
        self.pc.set_input(input, reset, load, inc);
    }

    pub fn tick(&mut self) {
        self.a.tick();
        self.d.tick();
        self.pc.tick();
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

    #[test]
    fn test_cpu_setting() {
        let mut cpu = Cpu::new();
        let new_value: u16 = 0xFFFF;

        cpu.set_a(new_value, true);
        cpu.set_d(new_value, true);
        cpu.set_pc(new_value, false, true, false);

        assert_eq!(cpu.get_a(), 0);
        assert_eq!(cpu.get_d(), 0);
        assert_eq!(cpu.get_pc(), 0);

        cpu.tick();

        assert_eq!(cpu.get_a(), new_value);
        assert_eq!(cpu.get_d(), new_value);
        assert_eq!(cpu.get_pc(), new_value);

    }
}
