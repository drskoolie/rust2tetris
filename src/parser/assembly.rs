use crate::parser::table::{
    SymbolTable,
    comp_table,
    dest_table,
    jump_table,
};

#[derive(Debug, PartialEq)]
pub enum AssemblyCommand {
    AInstruction(String),
    CInstruction(String),
    Label(String),
}

pub struct Assembler {
    pub symbol_table: SymbolTable,
    pub commands: Vec<AssemblyCommand>,
    pub next_variable_address: u16,
    pub binaries: Vec<String>,
}

impl Assembler {
    pub fn new() -> Self {
        Assembler { 
            symbol_table: SymbolTable::new(),
            commands: vec![],
            next_variable_address: 16,
            binaries: vec![],
        }
    }

    pub fn parse_source(&mut self, contents: &str) {
        self.commands = contents
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with("//"))
            .map(|line| {
                if line.starts_with('@') {
                    AssemblyCommand::AInstruction(line.strip_prefix('@').unwrap().to_string())
                } else if line.starts_with('(') && line.ends_with(')') {
                    AssemblyCommand::Label(
                        line.strip_prefix('(')
                            .and_then(|s| s.strip_suffix(')'))
                            .unwrap()
                            .to_string()
                    )
                } else if line.contains('=') || line.contains(';') {
                    AssemblyCommand::CInstruction(line.to_string())
                } else {
                    panic!("Invalid assembly instruction: {}", line);
                }
            })
            .collect()
    }

    pub fn assemble_a_instruction(&self, value: &str) -> String {
        let number: u16 = value.parse().unwrap_or_else(|_| {
            self.symbol_table.get_address(value)
                .unwrap_or_else(|| panic!("Symbol not found: {}", value))
        });
        format!("0{:015b}", number)

    }

    pub fn assemble_c_instruction(&self, value: &str) -> String {
        let mut dest = "";
        let comp;
        let mut jump = "";

        // Split on ';' first (jump part)
        let parts: Vec<&str> = value.split(';').collect();
        if parts.len() == 2 {
            jump = parts[1].trim();
        }

        // Split on '=' next (dest part)
        let eq_parts: Vec<&str> = parts[0].split('=').collect();
        if eq_parts.len() == 2 {
            dest = eq_parts[0].trim();
            comp = eq_parts[1].trim();
        } else {
            comp = eq_parts[0].trim();
        }

        let comp_map = comp_table();
        let dest_map = dest_table();
        let jump_map = jump_table();

        let (a_bit, c_bits) = comp_map.get(comp)
            .unwrap_or_else(|| panic!("Invalid comp field: {}", comp));

        let d_bits = dest_map.get(dest)
            .unwrap_or_else(|| panic!("Invalid dest field: {}", dest));

        let j_bits = jump_map.get(jump)
            .unwrap_or_else(|| panic!("Invalid jump field: {}", jump));

        format!("111{}{}{}{}", a_bit, c_bits, d_bits, j_bits)

    }

    pub fn resolve_symbols(&mut self) {
        let mut instruction_address = 0;

        // First pass: handle labels
        for command in &self.commands {
            match command {
                AssemblyCommand::Label(label) => {
                    self.symbol_table.add_entry(label, instruction_address);
                }
                _ => {
                    instruction_address += 1;
                }
            }
        }

        // Second pass: handle variables
        for command in &self.commands {
            if let AssemblyCommand::AInstruction(value) = command {
                if value.parse::<u16>().is_err() && !self.symbol_table.contains(value) {
                    self.symbol_table.add_entry(value, self.next_variable_address);
                    self.next_variable_address += 1;
                }
            }
        }
    }

    pub fn assemble_all(&mut self, contents: &str) {
        self.parse_source(contents);
        self.resolve_symbols();

        self.binaries = self.commands.iter().filter_map(|command| {
            match command {
                AssemblyCommand::AInstruction(value) => {
                    Some(self.assemble_a_instruction(value))
                }
                AssemblyCommand::CInstruction(value) => {
                    Some(self.assemble_c_instruction(value))
                }
                AssemblyCommand::Label(_) => None,
            }
        }).collect();

        self.binaries.push(format!("{:016b}", 0xFFFF)); 
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_assembly_valid_input() {
        let mut asm = Assembler::new();
        let source = r#"
            // This is a comment
            @15
            D=A
            @LOOP
            (LOOP)
            0;JMP

            // empty line below
            
        "#;

        asm.parse_source(source);

        let expected = vec![
            AssemblyCommand::AInstruction("15".to_string()),
            AssemblyCommand::CInstruction("D=A".to_string()),
            AssemblyCommand::AInstruction("LOOP".to_string()),
            AssemblyCommand::Label("LOOP".to_string()),
            AssemblyCommand::CInstruction("0;JMP".to_string()),
        ];

        assert_eq!(asm.commands, expected);
    }

    #[test]
    #[should_panic(expected = "Invalid assembly instruction")]
    fn test_parse_lines_invalid_input_panics() {
        let mut asm = Assembler::new();
        let source = "JUMP"; // does not contain '=' or ';'

        asm.parse_source(source);
    }

    #[test]
    fn test_assemble_a_instruction() {
        let asm = Assembler::new();
        let bin = asm.assemble_a_instruction("21");

        assert_eq!(bin, format!("{}{}{}{}", "0000", "0000", "0001", "0101"));
    }

    #[test]
    fn test_dest_equals_comp() {
        let asm = Assembler::new();
        let result = asm.assemble_c_instruction("D=A");
        assert_eq!(result, "1110110000010000"); // a=0, comp=A, dest=D, jump=null
    }

    #[test]
    fn test_comp_semicolon_jump() {
        let asm = Assembler::new();
        let result = asm.assemble_c_instruction("0;JMP");
        assert_eq!(result, "1110101010000111"); // a=0, comp=0, dest=null, jump=JMP
    }

    #[test]
    fn test_dest_equals_comp_jump() {
        let asm = Assembler::new();
        let result = asm.assemble_c_instruction("D=A+1;JGE");
        assert_eq!(result, format!("111{}{}{}{}", "0", "110111", "010", "011"));
    }

    #[test]
    fn test_full_m_form() {
        let asm = Assembler::new();
        let result = asm.assemble_c_instruction("AD=D+M;JNE");
        assert_eq!(result, format!("111{}{}{}{}", "1", "000010", "110", "101"));
    }

    #[test]
    fn test_comp_with_m() {
        let asm = Assembler::new();
        let result = asm.assemble_c_instruction("D=M");
        assert_eq!(result, format!("111{}{}{}{}", "1", "110000", "010", "000"));
    }

    #[test]
    #[should_panic(expected = "Invalid comp field")]
    fn test_invalid_comp_panics() {
        let asm = Assembler::new();
        asm.assemble_c_instruction("D=FOO");
    }

    #[test]
    #[should_panic(expected = "Invalid dest field")]
    fn test_invalid_dest_panics() {
        let asm = Assembler::new();
        asm.assemble_c_instruction("X=A");
    }

    #[test]
    #[should_panic(expected = "Invalid jump field")]
    fn test_invalid_jump_panics() {
        let asm = Assembler::new();
        asm.assemble_c_instruction("D=A;FLY");
    }

    #[test]
    fn test_predefined_symbols() {
        let table = SymbolTable::new();
        assert_eq!(table.get_address("SP"), Some(0));
        assert_eq!(table.get_address("R13"), Some(13));
        assert_eq!(table.get_address("SCREEN"), Some(16384));
        assert!(table.contains("KBD"));
        assert!(!table.contains("UNDECLARED"));
    }

    #[test]
    fn test_add_and_get() {
        let mut table = SymbolTable::new();
        table.add_entry("LOOP", 42);
        assert!(table.contains("LOOP"));
        assert_eq!(table.get_address("LOOP"), Some(42));
    }

    #[test]
    fn test_full_assembly_flow() {
        let source = r#"
            @i
            M=1
            (LOOP)
            @i
            D=M
            @100
            D=D-A
            @END
            D;JGT
            @LOOP
            0;JMP
            (END)
            @END
            0;JMP
        "#;

        let mut asm = Assembler::new();
        asm.assemble_all(source);

        assert_eq!(asm.binaries[0], "0000000000010000"); // @i = 16
        assert_eq!(asm.binaries[1], "1110111111001000"); // M=1
        assert_eq!(asm.binaries[2], "0000000000010000"); // @i
        assert_eq!(asm.binaries[3], "1111110000010000"); // D=M
        assert_eq!(asm.binaries[4], "0000000001100100"); // @100
        assert_eq!(asm.binaries[5], format!("111{}{}{}{}", "0", "010011", "010","000")); // D=D-A
        assert_eq!(asm.binaries[6], "0000000000001010"); // @END = 10
        assert_eq!(asm.binaries[7], "1110001100000001"); // D;JGT
        assert_eq!(asm.binaries[8], "0000000000000010"); // @LOOP = 2
        assert_eq!(asm.binaries[9], "1110101010000111"); // 0;JMP
        assert_eq!(asm.binaries[10], "0000000000001010"); // @END = 10
        assert_eq!(asm.binaries[11], "1110101010000111"); // 0;JMP
    }
}
