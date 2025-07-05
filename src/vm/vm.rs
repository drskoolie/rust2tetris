pub struct Stack {
    pub commands: Vec<String>,
    pub counter_eq: u16,
    pub counter_gt: u16,
    pub counter_lt: u16,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            commands: vec![],
            counter_eq: 0,
            counter_gt: 0,
            counter_lt: 0,
        }
    }

    pub fn pop_address(&mut self, address: usize) {
        let new_commands = vec![
            "@SP".to_string(), // A = SP Location
            "M=M-1".to_string(), // Decrement SP
            "A=M".to_string(), // A = SP Pointer
            "D=M".to_string(), // D = Ram[SP]
            format!("@{}", address), // A = Address Location
            "M=D".to_string(), // Ram[Address] = Ram[SP]
        ];

        self.commands.extend(new_commands);
    }

    pub fn push_value(&mut self, value: u16) {
        let new_commands = vec![
            format!("@{}", value), // Load constant into A
            "D=A".to_string(),    // D = constant
            "@SP".to_string(),    // A = SP location
            "A=M".to_string(),    // A = SP value (actual address)
            "M=D".to_string(),    // RAM[SP] = D = Constant
            "@SP".to_string(),    // A = SP location
            "M=M+1".to_string(),  // Increment SP
        ];

        self.commands.extend(new_commands);
    }

    fn setup_x_y(&mut self) {
        let new_commands = vec![
            // ** Get Y and save it in D //
            "@SP".to_string(),   // A = 0,  D = N/A
            "M=M-1".to_string(), // Go to correct location
            "A=M".to_string(),   // A = 257
            "D=M".to_string(),   // D = RAM[257]

            // ** Get X by setting A in the location of SP //
            "@SP".to_string(),   // A = 0, D = y
            "M=M-1".to_string(), // RAM[0] = 256
            "A=M".to_string(),   // A = 256
        ];

        self.commands.extend(new_commands);
    }

    fn push_result(&mut self) {
        let new_commands = vec![
            // ** push D //
            "M=D".to_string(),
            "@SP".to_string(),
            "M=M+1".to_string(),
        ];

        self.commands.extend(new_commands);
    }

    pub fn add(&mut self) {
        self.setup_x_y();

        let new_commands = vec![
            // M = X
            // D = Y
            "D=D+M".to_string(),
        ];
        self.commands.extend(new_commands);

        self.push_result();
    }

    pub fn sub(&mut self) {
        self.setup_x_y();

        let new_commands = vec![
            // M = X
            // D = Y
            "D=M-D".to_string(),
        ];
        self.commands.extend(new_commands);

        self.push_result();
    }

    pub fn and(&mut self) {
        self.setup_x_y();

        let new_commands = vec![
            // M = X
            // D = Y
            "D=D&M".to_string(),
        ];
        self.commands.extend(new_commands);

        self.push_result();
    }

    pub fn or(&mut self) {
        self.setup_x_y();

        let new_commands = vec![
            // M = X
            // D = Y
            "D=D|M".to_string(),
        ];
        self.commands.extend(new_commands);

        self.push_result();
    }

    pub fn not(&mut self) {
        let new_commands = vec![
            "@SP".to_string(), // A = 0, D = N/A
            "M=M-1".to_string(), // A = 0, D = N/A, RAM[0] = RAM[0]-1
            "A=M".to_string(), // A = RAM[SP]
            "D=!M".to_string(), // *sp is notted
        ];

        self.commands.extend(new_commands);

        self.push_result();

    }


    pub fn neg(&mut self) {
        let new_commands = vec![
            "@SP".to_string(), // A = 0, D = N/A
            "M=M-1".to_string(), // A = 0, D = N/A, RAM[0] = RAM[0]-1
            "A=M".to_string(), // A = RAM[SP]
            "D=-M".to_string(), // *sp is notted
        ];

        self.commands.extend(new_commands);

        self.push_result();

    }

    pub fn eq(&mut self) {
        let true_label = format!("EQ_TRUE_{}", self.counter_eq);
        let end_label = format!("EQ_END_{}", self.counter_eq);
        self.counter_eq += 1;
        self.setup_x_y();

        let new_commands = vec![
            // M = X
            // D = Y
            "D=M-D".to_string(),
            format!("@{}", true_label),
            "D;JEQ".to_string(), // If D == 0, jump to true, else go forward

            // ** Push FALSE and Jump to END
            "@SP".to_string(),
            "A=M".to_string(),
            "M=0".to_string(),
            format!("@{}", end_label),
            "0;JMP".to_string(),

            // ** Push TRUE and just continue to END
            format!("({})", true_label),
            "@SP".to_string(),
            "A=M".to_string(),
            "M=-1".to_string(),


            // ** END and increment SP
            format!("({})", end_label),
            "@SP".to_string(),
            "M=M+1".to_string(),
        ];

        self.commands.extend(new_commands);

    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::cpu::Cpu;
    use crate::parser::assembly::Assembler;

    #[test]
    fn test_stack_push_value_string() {
        let mut stack = Stack::new();
        stack.push_value(7);
        let expected = vec![
            "@7", "D=A", "@SP", "A=M", "M=D", "@SP", "M=M+1"
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>();

        assert_eq!(expected, stack.commands);
    }

    #[test]
    fn test_stack_push_value_cpu() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();
        stack.push_value(7);

        asm.assemble_all(&stack.commands.join("\n"));
        let no_of_instructions = asm.binaries.len();

        cpu.load_from_string(&asm.binaries.join("\n"));

        assert_eq!(256, cpu.get_data(0));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(257, cpu.get_data(0));
        assert_eq!(7, cpu.get_data(256));
    }

    #[test]
    fn test_stack_pop_address() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();
        let address: usize = 20;

        stack.push_value(7);
        stack.pop_address(address);
        asm.assemble_all(&stack.commands.join("\n"));
        let no_of_instructions = asm.binaries.len();

        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(256, cpu.get_data(0));
        assert_eq!(7, cpu.get_data(address));
    }

    #[test]
    fn test_stack_add() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        let val1 = 8;
        let val2 = 20;

        stack.push_value(val1);
        stack.push_value(val2);
        stack.add();
        asm.assemble_all(&stack.commands.join("\n"));
        let no_of_instructions = asm.binaries.len();

        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(257, cpu.get_data(0));
        assert_eq!(val1+val2, cpu.get_data(256));
    }

    #[test]
    fn test_stack_sub() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        let val1 = 25;
        let val2 = 20;

        stack.push_value(val1);
        stack.push_value(val2);
        stack.sub();
        asm.assemble_all(&stack.commands.join("\n"));
        let no_of_instructions = asm.binaries.len();

        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(257, cpu.get_data(0));
        assert_eq!(val1-val2, cpu.get_data(256));
    }

    #[test]
    fn test_stack_and() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        let val1 = 0b1010;
        let val2 = 0b1100;

        stack.push_value(val1);
        stack.push_value(val2);
        stack.and();
        asm.assemble_all(&stack.commands.join("\n"));
        let no_of_instructions = asm.binaries.len();

        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(257, cpu.get_data(0));
        assert_eq!(0b1000, cpu.get_data(256));
    }

    #[test]
    fn test_stack_or() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        let val1 = 0b1010;
        let val2 = 0b1100;

        stack.push_value(val1);
        stack.push_value(val2);
        stack.or();
        asm.assemble_all(&stack.commands.join("\n"));
        let no_of_instructions = asm.binaries.len();

        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(257, cpu.get_data(0));
        assert_eq!(0b1110, cpu.get_data(256));
    }

    #[test]
    fn test_stack_not() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        let val = 0x0000;

        stack.push_value(val);
        stack.not();
        asm.assemble_all(&stack.commands.join("\n"));
        let no_of_instructions = asm.binaries.len();

        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(257, cpu.get_data(0));
        assert_eq!(0xFFFF, cpu.get_data(256));
    }

    #[test]
    fn test_stack_neg() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        let val = 0b1;

        stack.push_value(val);
        stack.neg();
        asm.assemble_all(&stack.commands.join("\n"));
        let no_of_instructions = asm.binaries.len();

        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(257, cpu.get_data(0));
        assert_eq!(0xFFFF, cpu.get_data(256));
    }

    #[test]
    fn test_stack_eq_true() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        let val1 = 10;

        stack.push_value(val1);
        stack.push_value(val1);
        stack.eq();
        asm.assemble_all(&stack.commands.join("\n"));
        let no_of_instructions = asm.binaries.len();

        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(257, cpu.get_data(0));
        assert_eq!(0xFFFF, cpu.get_data(256));
    }



}
