pub struct Stack {
    pub assembly: Vec<String>,
    pub counter_eq: u16,
    pub counter_gt: u16,
    pub counter_lt: u16,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            assembly: vec![],
            counter_eq: 0,
            counter_gt: 0,
            counter_lt: 0,
        }
    }

    pub fn push_command(&mut self, segment: &str, number: &str)  {
        let index: u16 = number.parse().expect("Expected a number");

        match segment {
            "constant" => {
                self.push_value(index);
            }

            "local" => {
                self.push_from_pointer("LCL", index);
            }

            "argument" => {
                self.push_from_pointer("ARG", index);
            }

            "this" => {
                self.push_from_pointer("THIS", index);
            }

            "that" => {
                self.push_from_pointer("THAT", index);
            }

            "temp" => {
                let addr = 5 + index;
                let asm = vec![
                    format!("@{}", addr),
                    "D=M".to_string(),
                    "@SP".to_string(),
                    "A=M".to_string(),
                    "M=D".to_string(),
                    "@SP".to_string(),
                    "M=M+1".to_string(),
                ];
                self.assembly.extend(asm);
            }

            "pointer" => {
                let addr = match index {
                    0 => "THIS",
                    1 => "THAT",
                    _ => panic!("Invalid pointer index: {}", index),
                };
                let asm = vec![
                    format!("@{}", addr),
                    "D=M".to_string(),
                    "@SP".to_string(),
                    "A=M".to_string(),
                    "M=D".to_string(),
                    "@SP".to_string(),
                    "M=M+1".to_string(),
                ];
                self.assembly.extend(asm);
            }

            "static" => {
                let label = format!("Static.{}", index); // later you can prefix with filename
                let asm = vec![
                    format!("@{}", label),
                    "D=M".to_string(),
                    "@SP".to_string(),
                    "A=M".to_string(),
                    "M=D".to_string(),
                    "@SP".to_string(),
                    "M=M+1".to_string(),
                ];
                self.assembly.extend(asm);
            }

            _ => panic!("Unknown segment: {}", segment),

        }
    }

    pub fn push_from_pointer(&mut self, pointer: &str, index: u16) {
        let asm = vec![
            format!("@{}", pointer),
            "D=M".to_string(),
            format!("@{}", index),
            "A=D+A".to_string(),
            "D=M".to_string(),
            "@SP".to_string(),
            "A=M".to_string(),
            "M=D".to_string(),
            "@SP".to_string(),
            "M=M+1".to_string(),
        ];

        self.assembly.extend(asm);
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

        self.assembly.extend(new_commands);
    }

    pub fn pop_command(&mut self, segment: &str, number: &str) {
        let index: u16 = number.parse().expect("Expected a number");

        match segment {
            "local" => {
                self.pop_to_pointer("LCL", index);
            }

            "argument" => {
                self.pop_to_pointer("ARG", index);
            }

            "this" => {
                self.pop_to_pointer("THIS", index);
            }

            "that" => {
                self.pop_to_pointer("THAT", index);
            }

            "temp" => {
                let addr = 5 + index;
                let asm = vec![
                    "@SP".to_string(),
                    "M=M-1".to_string(),
                    "A=M".to_string(),
                    "D=M".to_string(),
                    format!("@{}", addr),
                    "M=D".to_string(),
                ];
                self.assembly.extend(asm);
            }

            "pointer" => {
                let addr = match index {
                    0 => "THIS",
                    1 => "THAT",
                    _ => panic!("Invalid pointer index: {}", index),
                };
                let asm = vec![
                    "@SP".to_string(),
                    "M=M-1".to_string(),
                    "A=M".to_string(),
                    "D=M".to_string(),
                    format!("@{}", addr),
                    "M=D".to_string(),
                ];
                self.assembly.extend(asm);
            }

            "static" => {
                let label = format!("Static.{}", index); // you may prefix with filename
                let asm = vec![
                    "@SP".to_string(),
                    "M=M-1".to_string(),
                    "A=M".to_string(),
                    "D=M".to_string(),
                    format!("@{}", label),
                    "M=D".to_string(),
                ];
                self.assembly.extend(asm);
            }

            "constant" => {
                panic!("Cannot pop to constant segment — it's not a memory region.");
            }

            _ => panic!("Unknown segment: {}", segment),
        }
    }

    pub fn pop_to_pointer(&mut self, pointer: &str, index: u16) {
        let asm = vec![
            // Compute address and store it in R13
            format!("@{}", pointer),
            "D=M".to_string(),
            format!("@{}", index),
            "D=D+A".to_string(),
            "@R13".to_string(),
            "M=D".to_string(),

            // Pop value into D
            "@SP".to_string(),
            "M=M-1".to_string(),
            "A=M".to_string(),
            "D=M".to_string(),

            // Store into address in R13
            "@R13".to_string(),
            "A=M".to_string(),
            "M=D".to_string(),
        ];

        self.assembly.extend(asm);
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

        self.assembly.extend(new_commands);
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

        self.assembly.extend(new_commands);
    }

    fn push_result(&mut self) {
        let new_commands = vec![
            // ** push D //
            "M=D".to_string(),
            "@SP".to_string(),
            "M=M+1".to_string(),
        ];

        self.assembly.extend(new_commands);
    }

    pub fn add(&mut self) {
        self.setup_x_y();

        let new_commands = vec![
            // M = X
            // D = Y
            "D=D+M".to_string(),
        ];
        self.assembly.extend(new_commands);

        self.push_result();
    }

    pub fn sub(&mut self) {
        self.setup_x_y();

        let new_commands = vec![
            // M = X
            // D = Y
            "D=M-D".to_string(),
        ];
        self.assembly.extend(new_commands);

        self.push_result();
    }

    pub fn and(&mut self) {
        self.setup_x_y();

        let new_commands = vec![
            // M = X
            // D = Y
            "D=D&M".to_string(),
        ];
        self.assembly.extend(new_commands);

        self.push_result();
    }

    pub fn or(&mut self) {
        self.setup_x_y();

        let new_commands = vec![
            // M = X
            // D = Y
            "D=D|M".to_string(),
        ];
        self.assembly.extend(new_commands);

        self.push_result();
    }

    pub fn not(&mut self) {
        let new_commands = vec![
            "@SP".to_string(), // A = 0, D = N/A
            "M=M-1".to_string(), // A = 0, D = N/A, RAM[0] = RAM[0]-1
            "A=M".to_string(), // A = RAM[SP]
            "D=!M".to_string(), // *sp is notted
        ];

        self.assembly.extend(new_commands);

        self.push_result();

    }


    pub fn neg(&mut self) {
        let new_commands = vec![
            "@SP".to_string(), // A = 0, D = N/A
            "M=M-1".to_string(), // A = 0, D = N/A, RAM[0] = RAM[0]-1
            "A=M".to_string(), // A = RAM[SP]
            "D=-M".to_string(), // *sp is notted
        ];

        self.assembly.extend(new_commands);

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

        self.assembly.extend(new_commands);

    }

    pub fn gt(&mut self) {
        let true_label = format!("GT_TRUE_{}", self.counter_gt);
        let end_label = format!("GT_END_{}", self.counter_gt);
        self.counter_gt += 1;
        self.setup_x_y();

        let new_commands = vec![
            // M = X
            // D = Y
            "D=M-D".to_string(),
            format!("@{}", true_label),
            "D;JGT".to_string(), // If D > 0, jump to true, else go forward

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

        self.assembly.extend(new_commands);

    }


    pub fn lt(&mut self) {
        let true_label = format!("GT_TRUE_{}", self.counter_gt);
        let end_label = format!("GT_END_{}", self.counter_gt);
        self.counter_gt += 1;
        self.setup_x_y();

        let new_commands = vec![
            // M = X
            // D = Y
            "D=M-D".to_string(),
            format!("@{}", true_label),
            "D;JLT".to_string(), // If D > 0, jump to true, else go forward

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

        self.assembly.extend(new_commands);

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

        assert_eq!(expected, stack.assembly);
    }

    #[test]
    fn test_stack_push_value_cpu() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();
        stack.push_value(7);

        asm.assemble_all(&stack.assembly.join("\n"));
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
        asm.assemble_all(&stack.assembly.join("\n"));
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
        asm.assemble_all(&stack.assembly.join("\n"));
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
        asm.assemble_all(&stack.assembly.join("\n"));
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
        asm.assemble_all(&stack.assembly.join("\n"));
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
        asm.assemble_all(&stack.assembly.join("\n"));
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
        asm.assemble_all(&stack.assembly.join("\n"));
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
        asm.assemble_all(&stack.assembly.join("\n"));
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
        asm.assemble_all(&stack.assembly.join("\n"));
        let no_of_instructions = asm.binaries.len();

        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(257, cpu.get_data(0));
        assert_eq!(0xFFFF, cpu.get_data(256));
    }

    #[test]
    fn test_stack_eq_false() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        let val1 = 10;
        let val2 = 11;

        stack.push_value(val1);
        stack.push_value(val2);
        stack.eq();
        asm.assemble_all(&stack.assembly.join("\n"));
        let no_of_instructions = asm.binaries.len();

        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(257, cpu.get_data(0));
        assert_eq!(0x0, cpu.get_data(256));
    }

    #[test]
    fn test_stack_eq_twice() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        let val = 10;

        stack.push_value(val);
        stack.push_value(val);
        stack.eq();
        stack.push_value(val);
        stack.push_value(val);
        stack.eq();
        asm.assemble_all(&stack.assembly.join("\n"));
        let no_of_instructions = asm.binaries.len();

        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(258, cpu.get_data(0));
        assert_eq!(0xFFFF, cpu.get_data(256));
        assert_eq!(0xFFFF, cpu.get_data(257));
    }


    #[test]
    fn test_stack_gt_true() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        let val1 = 10;

        stack.push_value(val1 + 1);
        stack.push_value(val1);
        stack.gt();
        asm.assemble_all(&stack.assembly.join("\n"));
        let no_of_instructions = asm.binaries.len();

        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(257, cpu.get_data(0));
        assert_eq!(0xFFFF, cpu.get_data(256));
    }

    #[test]
    fn test_stack_gt_false() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        let val1 = 10;
        let val2 = 11;

        stack.push_value(val1);
        stack.push_value(val2);
        stack.gt();
        asm.assemble_all(&stack.assembly.join("\n"));
        let no_of_instructions = asm.binaries.len();

        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(257, cpu.get_data(0));
        assert_eq!(0x0, cpu.get_data(256));
    }

    #[test]
    fn test_stack_gt_twice() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        let val = 10;

        stack.push_value(val+1);
        stack.push_value(val);
        stack.gt();
        stack.push_value(val);
        stack.push_value(val);
        stack.gt();
        asm.assemble_all(&stack.assembly.join("\n"));
        let no_of_instructions = asm.binaries.len();

        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(258, cpu.get_data(0));
        assert_eq!(0xFFFF, cpu.get_data(256));
        assert_eq!(0x0000, cpu.get_data(257));
    }


    #[test]
    fn test_stack_lt_true() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        let val1 = 10;

        stack.push_value(val1 - 1);
        stack.push_value(val1);
        stack.lt();
        asm.assemble_all(&stack.assembly.join("\n"));
        let no_of_instructions = asm.binaries.len();

        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(257, cpu.get_data(0));
        assert_eq!(0xFFFF, cpu.get_data(256));
    }

    #[test]
    fn test_stack_lt_false() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        let val1 = 10;

        stack.push_value(val1 + 1);
        stack.push_value(val1);
        stack.lt();
        asm.assemble_all(&stack.assembly.join("\n"));
        let no_of_instructions = asm.binaries.len();

        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(257, cpu.get_data(0));
        assert_eq!(0x0, cpu.get_data(256));
    }

    #[test]
    fn test_stack_lt_twice() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        let val = 10;

        stack.push_value(val-1);
        stack.push_value(val);
        stack.lt();
        stack.push_value(val);
        stack.push_value(val);
        stack.lt();
        asm.assemble_all(&stack.assembly.join("\n"));
        let no_of_instructions = asm.binaries.len();

        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(258, cpu.get_data(0));
        assert_eq!(0xFFFF, cpu.get_data(256));
        assert_eq!(0x0000, cpu.get_data(257));
    }

    #[test]
    fn test_push_constant_assembly() {
        let mut stack = Stack::new();
        stack.push_command("constant", "7");

        let expected = [
            "@7", "D=A", "@SP", "A=M", "M=D", "@SP", "M=M+1"
        ];

        assert_eq!(stack.assembly, expected.iter().map(|s| s.to_string()).collect::<Vec<_>>());
    }

    #[test]
    fn test_push_constant_once() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        stack.push_command("constant", "7");
        asm.assemble_all(&stack.assembly.join("\n"));
        let no_of_instructions = asm.binaries.len();

        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(257, cpu.get_data(0));
        assert_eq!(7, cpu.get_data(256));
    }

    #[test]
    fn test_push_constant_twice() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        stack.push_command("constant", "7");
        stack.push_command("constant", "15");
        asm.assemble_all(&stack.assembly.join("\n"));
        let no_of_instructions = asm.binaries.len();

        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(258, cpu.get_data(0));
        assert_eq!(7, cpu.get_data(256));
        assert_eq!(15, cpu.get_data(257));
    }

    #[test]
    fn test_pop_local() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        stack.push_command("constant", "25");
        stack.pop_command("local", "0");
        asm.assemble_all(&stack.assembly.join("\n"));
        let no_of_instructions = asm.binaries.len();
        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(256, cpu.get_data(0));
        assert_eq!(25, cpu.get_data(300));
    }

    #[test]
    fn test_pop_local_index() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        stack.push_command("constant", "25");
        stack.push_command("constant", "18");
        stack.pop_command("local", "1");
        stack.pop_command("local", "10");
        asm.assemble_all(&stack.assembly.join("\n"));
        let no_of_instructions = asm.binaries.len();
        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(256, cpu.get_data(0));
        assert_eq!(18, cpu.get_data(300 + 1));
        assert_eq!(25, cpu.get_data(300 + 10));
    }

    #[test]
    fn test_pop_argument() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        stack.push_command("constant", "25");
        stack.pop_command("argument", "0");
        asm.assemble_all(&stack.assembly.join("\n"));
        let no_of_instructions = asm.binaries.len();
        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(256, cpu.get_data(0));
        assert_eq!(25, cpu.get_data(400));
    }

    #[test]
    fn test_pop_argument_index() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        stack.push_command("constant", "25");
        stack.push_command("constant", "18");
        stack.pop_command("argument", "1");
        stack.pop_command("argument", "10");
        asm.assemble_all(&stack.assembly.join("\n"));
        let no_of_instructions = asm.binaries.len();
        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(256, cpu.get_data(0));
        assert_eq!(18, cpu.get_data(400 + 1));
        assert_eq!(25, cpu.get_data(400 + 10));
    }


    #[test]
    fn test_pop_this() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        stack.push_command("constant", "25");
        stack.pop_command("this", "0");
        asm.assemble_all(&stack.assembly.join("\n"));
        let no_of_instructions = asm.binaries.len();
        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(256, cpu.get_data(0));
        assert_eq!(25, cpu.get_data(3000));
    }

    #[test]
    fn test_pop_this_index() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        stack.push_command("constant", "25");
        stack.push_command("constant", "18");
        stack.pop_command("this", "1");
        stack.pop_command("this", "10");
        asm.assemble_all(&stack.assembly.join("\n"));
        let no_of_instructions = asm.binaries.len();
        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(256, cpu.get_data(0));
        assert_eq!(18, cpu.get_data(3000 + 1));
        assert_eq!(25, cpu.get_data(3000 + 10));
    }


    #[test]
    fn test_pop_that() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        stack.push_command("constant", "25");
        stack.pop_command("that", "0");
        asm.assemble_all(&stack.assembly.join("\n"));
        let no_of_instructions = asm.binaries.len();
        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(256, cpu.get_data(0));
        assert_eq!(25, cpu.get_data(3010));
    }

    #[test]
    fn test_pop_that_index() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        stack.push_command("constant", "25");
        stack.push_command("constant", "18");
        stack.pop_command("that", "1");
        stack.pop_command("that", "10");
        asm.assemble_all(&stack.assembly.join("\n"));
        let no_of_instructions = asm.binaries.len();
        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(256, cpu.get_data(0));
        assert_eq!(18, cpu.get_data(3010 + 1));
        assert_eq!(25, cpu.get_data(3010 + 10));
    }


    #[test]
    fn test_pop_temp() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        stack.push_command("constant", "25");
        stack.pop_command("temp", "0");
        asm.assemble_all(&stack.assembly.join("\n"));
        let no_of_instructions = asm.binaries.len();
        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(256, cpu.get_data(0));
        assert_eq!(25, cpu.get_data(5));
    }

    #[test]
    fn test_pop_temp_index() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        stack.push_command("constant", "25");
        stack.push_command("constant", "18");
        stack.pop_command("temp", "1");
        stack.pop_command("temp", "3");
        asm.assemble_all(&stack.assembly.join("\n"));
        let no_of_instructions = asm.binaries.len();
        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(256, cpu.get_data(0));
        assert_eq!(18, cpu.get_data(5 + 1));
        assert_eq!(25, cpu.get_data(5 + 3));
    }

    #[test]
    fn test_pop_pointer() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        stack.push_command("constant", "25");
        stack.push_command("constant", "15");
        stack.pop_command("pointer", "0");
        stack.pop_command("pointer", "1");
        asm.assemble_all(&stack.assembly.join("\n"));
        let no_of_instructions = asm.binaries.len();
        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(256, cpu.get_data(0));
        assert_eq!(15, cpu.get_data(3));
        assert_eq!(25, cpu.get_data(4));
    }

    #[test]
    fn test_pop_static_segment() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        // Push 42 and 99 onto the stack, then pop into Static.0 and Static.2
        stack.push_command("constant", "42");
        stack.push_command("constant", "99");
        stack.pop_command("static", "0");  // Static.0 = 99
        stack.pop_command("static", "2");  // Static.2 = 42

        asm.assemble_all(&stack.assembly.join("\n"));
        let no_of_instructions = asm.binaries.len();
        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        let addr1 = asm.symbol_table.get_address("Static.0").unwrap() as usize;
        let addr2 = asm.symbol_table.get_address("Static.2").unwrap() as usize;
        assert_eq!(256, cpu.get_data(0));         // SP back to base
        assert_eq!(99, cpu.get_data(addr1)); 
        assert_eq!(42, cpu.get_data(addr2));
    }

    #[test]
    fn test_push_segement_all() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let mut stack = Stack::new();

        stack.push_command("constant", "1");
        stack.pop_command("local", "0");
        stack.push_command("local", "0");

        stack.push_command("constant", "2");
        stack.pop_command("argument", "0");
        stack.push_command("argument", "0");

        stack.push_command("constant", "3");
        stack.pop_command("this", "0");
        stack.push_command("this", "0");

        stack.push_command("constant", "4");
        stack.pop_command("that", "0");
        stack.push_command("that", "0");

        stack.push_command("constant", "5");
        stack.pop_command("temp", "0");
        stack.push_command("temp", "0");

        stack.push_command("constant", "6");
        stack.pop_command("pointer", "0");
        stack.push_command("pointer", "0");

        stack.push_command("constant", "7");
        stack.pop_command("static", "7");
        stack.push_command("static", "7");

        asm.assemble_all(&stack.assembly.join("\n"));
        let no_of_instructions = asm.binaries.len();
        cpu.load_from_string(&asm.binaries.join("\n"));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(256 + 7, cpu.get_data(0));
        assert_eq!(1, cpu.get_data(256));
        assert_eq!(2, cpu.get_data(257));
        assert_eq!(3, cpu.get_data(258));
        assert_eq!(4, cpu.get_data(259));
        assert_eq!(5, cpu.get_data(260));
        assert_eq!(6, cpu.get_data(261));
        assert_eq!(7, cpu.get_data(262));
        assert_eq!(0, cpu.get_data(263));
    }

}
