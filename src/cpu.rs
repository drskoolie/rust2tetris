use crate::gates::{ get_bit };
use crate::alu::{ alu, AluFlags };
use crate::sequential::{ Register16, Counter16, Ram32K };

pub struct Cpu {
    pub a: Register16,
    pub d: Register16,
    pub pc: Counter16,
    pub data: Ram32K,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: Register16::new(),
            d: Register16::new(),
            pc: Counter16::new(),
            data: Ram32K::new(),
        }
    }

    pub fn get_a(&self) -> u16 {
        self.a.get_output()
    }

    pub fn set_a(&mut self, value: u16) {
        self.a.set_input(value, true);
    }

    pub fn print_a(&self) {
        println!{"A:  {:016b}", self.get_a()};
    }

    pub fn get_d(&self) -> u16 {
        self.d.get_output()
    }

    pub fn set_d(&mut self, value: u16) {
        self.d.set_input(value, true);
    }

    pub fn print_d(&self) {
        println!{"D:  {:016b}", self.get_d()};
    }

    pub fn get_pc(&self) -> u16 {
        self.pc.get_output()
    }

    pub fn set_pc(&mut self, input: u16, reset: bool, load: bool) {
        self.pc.set_input(input, reset, load, false);
    }

    pub fn inc_pc(&mut self) {
        self.pc.set_input(0x0, false, false, true);
    }

    pub fn print_pc(&self) {
        println!{"PC: {:016b}", self.get_pc()};
    }

    pub fn print_cpu(&self) {
        println!{"------------"};
        self.print_a();
        self.print_d();
        self.print_pc();
    }

    pub fn tick(&mut self) {
        self.a.tick();
        self.d.tick();
        self.pc.tick();
        self.data.tick();
    }

    pub fn execute(&mut self, instruction: u16) {
        let is_c_instruction = get_bit(instruction, 15);

        if is_c_instruction {
            let is_memory = get_bit(instruction, 12);
            let a_register = if is_memory{
                let address = self.get_a() as usize;
                self.data.get(address)
            } else {
                self.get_a()
            };

            let flags_alu = AluFlags {
                zx: get_bit(instruction, 11),
                nx: get_bit(instruction, 10),
                zy: get_bit(instruction, 9),
                ny: get_bit(instruction, 8),
                f:  get_bit(instruction, 7),
                no: get_bit(instruction, 6),
            };

            let (output, zr, ng) = alu(self.get_d(), a_register, flags_alu);

            let d1 = get_bit(instruction, 5);
            let d2 = get_bit(instruction, 4);
            let d3 = get_bit(instruction, 3);

            if d3 {
                self.data.set(self.get_a() as usize, output);
            }

            if d2 {
                self.set_d(output);
            }

            if d1 {
                self.set_a(output);
            }

        } else {
            self.set_a(instruction);
        }

        self.tick();

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

        cpu.set_a(new_value);
        cpu.set_d(new_value);
        cpu.set_pc(new_value, false, true);

        assert_eq!(cpu.get_a(), 0);
        assert_eq!(cpu.get_d(), 0);
        assert_eq!(cpu.get_pc(), 0);

        cpu.tick();

        assert_eq!(cpu.get_a(), new_value);
        assert_eq!(cpu.get_d(), new_value);
        assert_eq!(cpu.get_pc(), new_value);
    }

    #[test]
    fn test_a_instruction() {
        let mut cpu = Cpu::new();
        let instruction: u16 = 0x7FFF;

        cpu.execute(instruction);
        assert_eq!(cpu.get_a(), instruction);
    }
}
