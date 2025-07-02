use crate::gates::get_bit;
use crate::alu::{ alu, AluFlags };
use crate::memory::{ 
    Register16,
    Counter16,
    Ram16K,
    Rom32K,
};

pub struct Cpu {
    a: Register16,
    d: Register16,
    pc: Counter16,
    data: Ram16K,
    rom: Rom32K,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: Register16::new(),
            d: Register16::new(),
            pc: Counter16::new(),
            data: Ram16K::new(),
            rom: Rom32K::new(),
        }
    }

    pub fn get_a(&self) -> u16 {
        self.a.get()
    }

    pub fn set_a(&mut self, value: u16) {
        self.a.set(value);
    }

    pub fn print_a(&self) {
        println!{"A:  {:016b}", self.get_a()};
    }

    pub fn get_d(&self) -> u16 {
        self.d.get()
    }

    pub fn set_d(&mut self, value: u16) {
        self.d.set(value);
    }

    pub fn print_d(&self) {
        println!{"D:  {:016b}", self.get_d()};
    }

    pub fn get_pc(&self) -> u16 {
        self.pc.get()
    }

    pub fn set_pc(&mut self, input: u16) {
        self.pc.set(input);
    }

    pub fn inc_pc(&mut self) {
        self.pc.inc();
    }

    pub fn reset_pc(&mut self) {
        self.pc.reset();
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

    pub fn load_from_file(&mut self, path: &str) {
        self.rom.load_from_file(path);
    }

    pub fn fetch(&self) -> u16 {
        let address = self.get_pc() as usize;
        self.rom.get(address)
    }

    pub fn execute(&mut self, instruction: u16) {
        let is_c_instruction = get_bit(instruction, 15);
        let mut jump = false;

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

            let (output, is_zero, is_neg) = alu(self.get_d(), a_register, flags_alu);
            let is_pos = !is_zero && !is_neg;

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

            let jump_code: u8 = 
                ((get_bit(instruction, 2) as u8) << 2) |
                ((get_bit(instruction, 1) as u8) << 1) |
                (get_bit(instruction, 0) as u8);

            jump = match jump_code {
                0b000 => false,                              // null
                0b001 => is_pos,                             // JGT
                0b010 => is_zero,                            // JEQ
                0b011 => is_pos || is_zero,                  // JGE
                0b100 => is_neg,                             // JLT
                0b101 => !is_zero,                           // JNE
                0b110 => is_neg || is_zero,                  // JLE
                0b111 => true,                               // JMP
                _ => false,                                  // should never happen
            };


        } else {
            self.set_a(instruction);
        }

        if jump {
            self.set_pc(self.get_a());
        } else {
            self.inc_pc();
        }
    }

    pub fn clock(&mut self) {
        let instruction = self.fetch();
        self.execute(instruction);
        self.tick();
    }

    pub fn run(&mut self) {
        loop {
            self.clock();
            self.print_cpu();
        }
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
    fn test_cpu_pc() {
        let mut cpu = Cpu::new();
        let value: u16 = 0xFF00;

        cpu.set_pc(value);
        cpu.tick();
        assert_eq!{cpu.get_pc(), value};

        cpu.inc_pc();
        cpu.tick();
        assert_eq!{cpu.get_pc(), value.wrapping_add(1)};

        cpu.reset_pc();
        cpu.tick();
        assert_eq!{cpu.get_pc(), 0};
    }

    #[test]
    fn test_cpu_setting() {
        let mut cpu = Cpu::new();
        let new_value: u16 = 0xFFFF;

        cpu.set_a(new_value);
        cpu.set_d(new_value);
        cpu.set_pc(new_value);

        assert_eq!(cpu.get_a(), 0);
        assert_eq!(cpu.get_d(), 0);
        assert_eq!(cpu.get_pc(), 0);

        cpu.tick();

        assert_eq!(cpu.get_a(), new_value);
        assert_eq!(cpu.get_d(), new_value);
        assert_eq!(cpu.get_pc(), new_value);
    }

    #[test]
    fn test_cpu_a_instruction() {
        let mut cpu = Cpu::new();
        let instruction: u16 = 0x7FFF;

        cpu.execute(instruction);
        cpu.tick();
        assert_eq!(cpu.get_a(), instruction);
    }

    #[test]
    fn test_cpu_zero_a() {
        let mut cpu = Cpu::new();
        // a: 0
        // c: 101010
        // d: 010
        let instruction: u16 = 0b1110_1010_1001_0000; 

        cpu.set_d(10);
        cpu.tick();
        assert_eq!{cpu.get_d(), 10};
        cpu.execute(instruction);
        cpu.tick();
        assert_eq!{cpu.get_d(), 0};
    }

    #[test]
    fn test_cpu_one() {
        let mut cpu = Cpu::new();
        // a: 0
        // c: 111111
        // d: 010
        let instruction: u16 = 0b1110_1111_1101_0000; 

        cpu.set_d(10);
        cpu.tick();
        assert_eq!{cpu.get_d(), 10};
        cpu.execute(instruction);
        cpu.tick();
        assert_eq!{cpu.get_d(), 1};
    }

    #[test]
    fn test_cpu_neg_one() {
        let mut cpu = Cpu::new();
        // a: 0
        // c: 111010
        // d: 010
        let instruction: u16 = 0b1110_1110_1001_0000; 

        cpu.set_d(10);
        cpu.tick();
        assert_eq!{cpu.get_d(), 10};
        cpu.execute(instruction);
        cpu.tick();
        assert_eq!{cpu.get_d(), (!0x0001u16).wrapping_add(1)};
    }

    #[test]
    fn test_cpu_out_d() {
        let mut cpu = Cpu::new();
        // a: 0
        // c: 001100
        // d: 010

        let instruction: u16 = 0b1110_0011_0001_0000;

        cpu.set_a(10);
        cpu.set_d(15);
        cpu.tick();
        assert_eq!{cpu.get_a(), 10};
        assert_eq!{cpu.get_d(), 15};
        cpu.execute(instruction);
        cpu.tick();
        assert_eq!{cpu.get_d(), 15};
    }

    #[test]
    fn test_cpu_out_a() {
        let mut cpu = Cpu::new();
        // a: 0
        // c: 110000
        // d: 010

        let instruction: u16 = 0b1110_1100_0001_0000;

        cpu.set_a(10);
        cpu.set_d(15);
        cpu.tick();
        assert_eq!{cpu.get_a(), 10};
        assert_eq!{cpu.get_d(), 15};
        cpu.execute(instruction);
        cpu.tick();
        assert_eq!{cpu.get_d(), 10};
    }

    #[test]
    fn test_cpu_jump_a_instruction() {
        let mut cpu = Cpu::new();
        let instruction: u16 = 0x0001;

        assert_eq!(cpu.get_pc(), 0);
        cpu.execute(instruction);
        cpu.tick();
        assert_eq!(cpu.get_pc(), 1);
    }

    #[test]
    fn test_cpu_jump_c_instruction_null() {
        let mut cpu = Cpu::new();
        let instruction: u16 = 0b1111_0000_0000_0000;
        let memory_loc: u16 = 0x2FFF;

        cpu.execute(memory_loc);
        cpu.tick();
        assert_eq!{cpu.get_a(), memory_loc};

        assert_eq!(cpu.get_pc(), 1);
        cpu.execute(instruction);
        cpu.tick();
        assert_eq!(cpu.get_pc(), 2);
    }

    #[test]
    #[allow(clippy::unusual_byte_groupings)]
    fn test_cpu_jump_c_instruction_jgt() {
        let mut cpu = Cpu::new();
        // a: 0
        // c: 111111
        // d: 010
        // j: 001
        let instruction: u16 = 0b111_0_111111_010_001;
        let memory_loc: u16 = 0x7FFF;

        assert_eq!{cpu.get_pc(), 0};
        cpu.execute(memory_loc);
        cpu.tick();
        assert_eq!{cpu.get_a(), memory_loc};
        assert_eq!{cpu.get_pc(), 1};

        cpu.execute(instruction);
        cpu.tick();
        assert_eq!{cpu.get_d(), 1};
        assert_eq!{cpu.get_pc(), memory_loc};
    }

    #[test]
    #[allow(clippy::unusual_byte_groupings)]
    fn test_cpu_jump_c_instruction_jeq() {
        let mut cpu = Cpu::new();
        // a: 0
        // c: 101010 // Set to Zero
        // d: 010
        // j: 010
        let instruction: u16 = 0b111_0_101010_010_010;
        let memory_loc: u16 = 0x7FFF;

        assert_eq!{cpu.get_pc(), 0};
        cpu.execute(memory_loc);
        cpu.tick();
        assert_eq!{cpu.get_a(), memory_loc};
        assert_eq!{cpu.get_pc(), 1};

        cpu.execute(instruction);
        cpu.tick();
        assert_eq!{cpu.get_d(), 0};
        assert_eq!{cpu.get_pc(), memory_loc};
    }

    #[test]
    #[allow(clippy::unusual_byte_groupings)]
    fn test_cpu_jump_c_instruction_jge() {
        let mut cpu = Cpu::new();
        // c: 101010 (0)
        // d: 010
        // j: 011
        let instruction: u16 = 0b111_0_101010_010_011;
        let memory_loc: u16 = 0x7FFF;

        assert_eq!{cpu.get_pc(), 0};
        cpu.execute(memory_loc);
        cpu.tick();
        assert_eq!{cpu.get_a(), memory_loc};
        assert_eq!{cpu.get_pc(), 1};

        cpu.execute(instruction);
        cpu.tick();
        assert_eq!{cpu.get_d(), 0};
        assert_eq!{cpu.get_pc(), memory_loc};
    }

    #[test]
    #[allow(clippy::unusual_byte_groupings)]
    fn test_cpu_jump_c_instruction_jlt() {
        let mut cpu = Cpu::new();
        // c: 111010 (−1)
        // d: 010
        // j: 100
        let instruction: u16 = 0b111_0_111010_010_100;
        let memory_loc: u16 = 0x7FFF;

        assert_eq!{cpu.get_pc(), 0};
        cpu.execute(memory_loc);
        cpu.tick();
        assert_eq!{cpu.get_a(), memory_loc};
        assert_eq!{cpu.get_pc(), 1};

        cpu.execute(instruction);
        cpu.tick();
        assert_eq!{cpu.get_d(), 0xFFFF}; // −1
        assert_eq!{cpu.get_pc(), memory_loc};
    }

    #[test]
    #[allow(clippy::unusual_byte_groupings)]
    fn test_cpu_jump_c_instruction_jne() {
        let mut cpu = Cpu::new();
        // c: 111111 (1)
        // d: 010
        // j: 101
        let instruction: u16 = 0b111_0_111111_010_101;
        let memory_loc: u16 = 0x7FFF;

        assert_eq!{cpu.get_pc(), 0};
        cpu.execute(memory_loc);
        cpu.tick();
        assert_eq!{cpu.get_a(), memory_loc};
        assert_eq!{cpu.get_pc(), 1};

        cpu.execute(instruction);
        cpu.tick();
        assert_eq!{cpu.get_d(), 1};
        assert_eq!{cpu.get_pc(), memory_loc};
    }

    #[test]
    #[allow(clippy::unusual_byte_groupings)]
    fn test_cpu_jump_c_instruction_jle() {
        let mut cpu = Cpu::new();
        // c: 111010 (−1)
        // d: 010
        // j: 110
        let instruction: u16 = 0b111_0_111010_010_110;
        let memory_loc: u16 = 0x7FFF;

        assert_eq!{cpu.get_pc(), 0};
        cpu.execute(memory_loc);
        cpu.tick();
        assert_eq!{cpu.get_a(), memory_loc};
        assert_eq!{cpu.get_pc(), 1};

        cpu.execute(instruction);
        cpu.tick();
        assert_eq!{cpu.get_d(), 0xFFFF}; // −1
        assert_eq!{cpu.get_pc(), memory_loc};
    }

    #[test]
    #[allow(clippy::unusual_byte_groupings)]
    fn test_cpu_jump_c_instruction_jmp() {
        let mut cpu = Cpu::new();
        // c: 101010 (0)
        // d: 010
        // j: 111
        let instruction: u16 = 0b111_0_101010_010_111;
        let memory_loc: u16 = 0x7FFF;

        assert_eq!{cpu.get_pc(), 0};
        cpu.execute(memory_loc);
        cpu.tick();
        assert_eq!{cpu.get_a(), memory_loc};
        assert_eq!{cpu.get_pc(), 1};

        cpu.execute(instruction);
        cpu.tick();
        assert_eq!{cpu.get_d(), 0};
        assert_eq!{cpu.get_pc(), memory_loc};
    }

}
