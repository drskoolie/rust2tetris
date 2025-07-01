use std::array::from_fn;
use crate::gates::{ mux16, inc16} ;

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

    pub fn set_input(&mut self, input: u16, load: bool) {
        self.dff.set_input(mux16(input, self.dff.get_output(), load));
    }

    pub fn get_output(&self) -> u16 {
        self.dff.get_output()
    }

    pub fn tick(&mut self) {
        self.dff.tick();
    }
}

pub struct Counter16 {
    dff: Dff,
}

impl Counter16 {
    pub fn new() -> Self {
        Counter16 { dff: Dff::new() }
    }

    pub fn set_input(&mut self, input: u16, reset: bool, load: bool, inc: bool) {
        if reset {
            self.dff.set_input(0x0);
        } else if load { 
            self.dff.set_input(input);
        } else if inc {
            self.dff.set_input(inc16(self.dff.get_output()));
        }
    }

    pub fn get_output(&self) -> u16 {
        self.dff.get_output()
    }

    pub fn tick(&mut self) {
        self.dff.tick();
    }
}

pub struct Ram32K {
    registers: [Register16; 32 * 1024], // 32K = 32768
}

impl Ram32K {
    pub fn new() -> Self {
        Ram32K {
            registers: from_fn(|_| Register16::new()),
        }
    }

    pub fn get(&self, address: usize) -> u16 {
        assert!(address < 32 * 1024);
        self.registers[address].get_output()
    }

    pub fn set(&mut self, address: usize, value: u16) {
        assert!(address < 32 * 1024);
        self.registers[address].set_input(value, true);
    }

    pub fn tick(&mut self) {
        for reg in self.registers.iter_mut() {
            reg.tick();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(reg.get_output(), 0x0);

        let input1 = 0xAAAA;
        reg.set_input(input1, true);
        assert_eq!(reg.get_output(), 0x0);
        reg.tick() ;
        assert_eq!(reg.get_output(), input1);

        let input2 = 0xBCBC;
        reg.set_input(input2, false);
        reg.tick();
        assert_eq!(reg.get_output(), input1);
        reg.set_input(input2, true);
        reg.tick();
        assert_eq!(reg.get_output(), input2);

    }

    #[test]
    fn test_counter_init() {
        let counter = Counter16::new();

        assert_eq!(counter.get_output(), 0x0);
    }

    #[test]
    fn test_counter_load() {
        let mut counter = Counter16::new();
        let input = 0x10F0;

        assert_eq!(counter.get_output(), 0x0);
        counter.set_input(input, false, true, false);
        assert_eq!(counter.get_output(), 0x0);
        counter.tick();
        assert_eq!(counter.get_output(), input);
    }

    #[test]
    fn test_counter_reset() {
        let mut counter = Counter16::new();
        let input = 0x10F0;

        assert_eq!(counter.get_output(), 0x0);
        counter.set_input(input, false, true, false);
        assert_eq!(counter.get_output(), 0x0);
        counter.tick();
        assert_eq!(counter.get_output(), input);

        counter.set_input(input, true, false, false);
        assert_eq!(counter.get_output(), input);
        counter.tick();
        assert_eq!(counter.get_output(), 0x0);
    }

    #[test]
    fn test_counter_inc() {
        let mut counter = Counter16::new();
        let input = 0x10F0;

        assert_eq!(counter.get_output(), 0x0);
        counter.set_input(input, false, false, true);
        counter.tick();
        assert_eq!(counter.get_output(), 1);
    }

    #[test]
    fn test_ram32k_initial_state() {
        let ram = Ram32K::new();
        for addr in [0, 1, 100, 1023, 32767] {
            assert_eq!(ram.get(addr), 0);
        }
    }

    #[test]
    fn test_ram32k_write_and_read_single_address() {
        let mut ram = Ram32K::new();
        let addr = 12345;
        let value = 0xBEEF;

        // Write value with load = true
        ram.set(addr, value);
        assert_eq!(ram.get(addr), 0); // Not yet stored until tick

        ram.tick(); // Now commit
        assert_eq!(ram.get(addr), value);
    }

    #[test]
    fn test_ram32k_multiple_addresses_independent() {
        let mut ram = Ram32K::new();
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
    fn test_ram32k_write_same_address_multiple_times() {
        let mut ram = Ram32K::new();
        let addr = 30000;

        ram.set(addr, 0xAAAA);
        ram.tick();
        assert_eq!(ram.get(addr), 0xAAAA);

        ram.set(addr, 0xBBBB);
        ram.tick();
        assert_eq!(ram.get(addr), 0xBBBB);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_ram32k_out_of_bounds_get() {
        let ram = Ram32K::new();
        ram.get(32768); // Invalid index
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_ram32k_out_of_bounds_set() {
        let mut ram = Ram32K::new();
        ram.set(40000, 0x1234); // Invalid index
    }


}
