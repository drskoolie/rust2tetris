use std::fs::File;
use std::io::{BufRead, BufReader};
use std::array::from_fn;

pub struct Dff {
    input: u16,
    output: u16,
}

impl Dff {
    pub fn new() -> Self {
        Dff { input: 0x0000, output: 0x0000 }
    }

    pub fn set_input(&mut self, value: u16) {
        self.input = value;
    }

    pub fn get_output(&self) -> u16 {
        self.output
    }

    pub fn tick(&mut self) {
        self.output = self.input;
    }
}

pub struct Register16 {
    dff: Dff,
}

impl Register16 {
    pub fn new() -> Self {
        Register16 { dff: Dff::new() }
    }

    pub fn set(&mut self, input: u16) {
        self.dff.set_input(input);
    }

    pub fn get(&self) -> u16 {
        self.dff.get_output()
    }

    pub fn tick(&mut self) {
        self.dff.tick();
    }
}

pub struct Counter16 {
    reg: Register16,
}

impl Counter16 {
    pub fn new() -> Self {
        Counter16 { reg: Register16::new() }
    }

    pub fn set(&mut self, input: u16) {
        self.reg.set(input);
    }

    pub fn reset(&mut self) {
        self.reg.set(0x0);
    }

    pub fn inc(&mut self) {
        self.reg.set(self.reg.get().wrapping_add(1));
    }

    pub fn get(&self) -> u16 {
        self.reg.get()
    }

    pub fn tick(&mut self) {
        self.reg.tick();
    }
}


pub struct Ram16K {
    registers: [Register16; 16 * 1024], // 16K = 16384
}

impl Ram16K {
    pub fn new() -> Self {
        Ram16K {
            registers: from_fn(|_| Register16::new()),
        }
    }

    pub fn get(&self, address: usize) -> u16 {
        assert!(address < 16 * 1024);
        self.registers[address].get()
    }

    pub fn set(&mut self, address: usize, value: u16) {
        assert!(address < 16 * 1024);
        self.registers[address].set(value);
    }

    pub fn tick(&mut self) {
        for reg in self.registers.iter_mut() {
            reg.tick();
        }
    }
}

pub struct Rom32K {
    registers: [Register16; 32 * 1024], // 32K = 32768
}

impl Rom32K {
    pub fn new() -> Self {
        Rom32K {
            registers: from_fn(|_| Register16::new()),
        }
    }

    pub fn set(&mut self, address: usize, value: u16) {
        assert!(address < 32 * 1024);
        self.registers[address].set(value);
    }

    pub fn get(&self, address: usize) -> u16 {
        assert!(address < 32 * 1024);
        self.registers[address].get()
    }

    pub fn tick(&mut self) {
        for reg in self.registers.iter_mut() {
            reg.tick();
        }
    }

    pub fn load_from_file(&mut self, path: &str) {
        let file = File::open(path).expect("Failed to open ROM File");
        let reader = BufReader::new(file);

        for (i, line) in reader.lines().enumerate() {
            if i >= 32 * 1024 {
                panic!("ROM file exceeds 32K instruction limit.");
            }
            let line = line.unwrap_or_else(|_| panic!("Failed reading line {} in file '{}'", i, path));
            if line.trim().is_empty() {
                continue;
            }

            let instruction = u16::from_str_radix(&line, 2)
                .unwrap_or_else(|_| panic!("Invalid binary '{}' in file '{}', line {}", line, path, i));

            self.set(i, instruction);
        }

        self.tick();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;
    use std::path::Path;

    #[test]
    fn test_dff_basic_behavior() {
        let mut dff = Dff::new();

        // Initially output should be 0
        assert_eq!(dff.get_output(), 0);

        // Set input but don't tick — output should remain 0
        dff.set_input(0xFFFF);
        assert_eq!(dff.get_output(), 0);

        // Now tick — output should update to input
        dff.tick();
        assert_eq!(dff.get_output(), 0xFFFF);
    }

    #[test]
    fn test_dff_multiple_ticks() {
        let mut dff = Dff::new();

        dff.set_input(0xAAAA);
        dff.tick();
        assert_eq!(dff.get_output(), 0xAAAA);

        dff.set_input(0x5555);
        assert_eq!(dff.get_output(), 0xAAAA); // Still old output

        dff.tick();
        assert_eq!(dff.get_output(), 0x5555);
    }

    #[test]
    fn test_register_behavior() {
        let mut reg = Register16::new();
        assert_eq!(reg.get(), 0x0);

        let input1 = 0xAAAA;
        reg.set(input1);
        assert_eq!(reg.get(), 0x0);
        reg.tick() ;
        assert_eq!(reg.get(), input1);

        let input2 = 0xBCBC;
        reg.set(input2);
        reg.tick();
        assert_eq!(reg.get(), input2);

    }

    #[test]
    fn test_counter_init() {
        let counter = Counter16::new();

        assert_eq!(counter.get(), 0x0);
    }

    #[test]
    fn test_counter_set() {
        let mut counter = Counter16::new();
        let input = 0x10F0;

        assert_eq!(counter.get(), 0x0);
        counter.set(input);
        assert_eq!(counter.get(), 0x0);
        counter.tick();
        assert_eq!(counter.get(), input);
    }

    #[test]
    fn test_counter_reset() {
        let mut counter = Counter16::new();
        let input = 0x10F0;

        counter.set(input);
        counter.tick();
        assert_eq!(counter.get(), input);
        counter.reset();
        counter.tick();
        assert_eq!(counter.get(), 0x0);
    }

    #[test]
    fn test_counter_inc() {
        let mut counter = Counter16::new();

        assert_eq!(counter.get(), 0x0);
        counter.inc();
        counter.tick();
        assert_eq!(counter.get(), 1);
    }

    #[test]
    fn test_ram16k_initial_state() {
        let ram = Ram16K::new();
        for addr in [0, 1, 100, 1023, 16000] {
            assert_eq!(ram.get(addr), 0);
        }
    }

    #[test]
    fn test_ram16k_write_and_read_single_address() {
        let mut ram = Ram16K::new();
        let addr = 0x2FFF;
        let value = 0xBEEF;

        // Write value with load = true
        ram.set(addr, value);
        assert_eq!(ram.get(addr), 0); // Not yet stored until tick

        ram.tick(); // Now commit
        assert_eq!(ram.get(addr), value);
    }

    #[test]
    fn test_ram16k_multiple_addresses_independent() {
        let mut ram = Ram16K::new();
        let addr1 = 100;
        let addr2 = 200;
        let val1 = 0x1111;
        let val2 = 0x2222;

        ram.set(addr1, val1);
        ram.set(addr2, val2);
        ram.tick();

        assert_eq!(ram.get(addr1), val1);
        assert_eq!(ram.get(addr2), val2);
    }

    #[test]
    fn test_ram16k_write_same_address_multiple_times() {
        let mut ram = Ram16K::new();
        let addr = 15000;

        ram.set(addr, 0xAAAA);
        ram.tick();
        assert_eq!(ram.get(addr), 0xAAAA);

        ram.set(addr, 0xBBBB);
        ram.tick();
        assert_eq!(ram.get(addr), 0xBBBB);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_ram16k_out_of_bounds_get() {
        let ram = Ram16K::new();
        ram.get(32768); // Invalid index
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_ram16k_out_of_bounds_set() {
        let mut ram = Ram16K::new();
        ram.set(40000, 0x1234); // Invalid index
    }

    #[test]
    fn test_rom32k_basic() {
        let mut rom = Rom32K::new();
        let address: usize = 0;

        rom.set(address, 10);
        rom.tick();
        assert_eq!(rom.get(address), 10);
    }

}
