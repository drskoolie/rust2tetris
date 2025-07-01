use crate::gates::{ mux16, inc16} ;

pub struct DFF {
    input: u16,
    output: u16,
}

impl DFF {
    pub fn new() -> Self {
        DFF { input: 0x0000, output: 0x0000 }
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
    dff: DFF,
}

impl Register16 {
    pub fn new() -> Self {
        Register16 { dff: DFF::new() }
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
    dff: DFF,
}

impl Counter16 {
    pub fn new() -> Self {
        Counter16 { dff: DFF::new() }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dff_basic_behavior() {
        let mut dff = DFF::new();

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
        let mut dff = DFF::new();

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

}
