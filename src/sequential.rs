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
}
