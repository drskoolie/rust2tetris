use std::collections::HashMap;

fn comp_table() -> HashMap<&'static str, (&'static str, &'static str)> {
    // returns (a-bit, c-bits)
    HashMap::from([
        ("0",    ("0", "101010")),
        ("1",    ("0", "111111")),
        ("-1",   ("0", "111010")),
        ("D",    ("0", "001100")),
        ("A",    ("0", "110000")),
        ("!D",   ("0", "001101")),
        ("!A",   ("0", "110001")),
        ("-D",   ("0", "001111")),
        ("-A",   ("0", "110011")),
        ("D+1",  ("0", "011111")),
        ("A+1",  ("0", "110111")),
        ("D-1",  ("0", "001110")),
        ("A-1",  ("0", "110010")),
        ("D+A",  ("0", "000010")),
        ("D-A",  ("0", "010011")),
        ("A-D",  ("0", "000111")),
        ("D&A",  ("0", "000000")),
        ("D|A",  ("0", "010101")),
        ("M",    ("1", "110000")),
        ("!M",   ("1", "110001")),
        ("-M",   ("1", "110011")),
        ("M+1",  ("1", "110111")),
        ("M-1",  ("1", "110010")),
        ("D+M",  ("1", "000010")),
        ("D-M",  ("1", "010011")),
        ("M-D",  ("1", "000111")),
        ("D&M",  ("1", "000000")),
        ("D|M",  ("1", "010101")),
    ])
}

fn dest_table() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("",    "000"),
        ("M",   "001"),
        ("D",   "010"),
        ("MD",  "011"),
        ("A",   "100"),
        ("AM",  "101"),
        ("AD",  "110"),
        ("AMD", "111"),
    ])
}

fn jump_table() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("",    "000"),
        ("JGT", "001"),
        ("JEQ", "010"),
        ("JGE", "011"),
        ("JLT", "100"),
        ("JNE", "101"),
        ("JLE", "110"),
        ("JMP", "111"),
    ])
}


#[derive(Debug, PartialEq)]
enum AssemblyCommand {
    AInstruction(String),
    CInstruction(String),
    Label(String),
}


fn parse_assembly(contents: &str) -> Vec<AssemblyCommand> {
    contents
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

fn assemble_a_instruction(value: &str) -> String {
    let number: u16 = value.parse().expect("Expected numeric A-instruction");
    format!("0{:015b}", number)
}

fn assemble_c_instruction(value: &str) -> String {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_assembly_valid_input() {
        let input = r#"
            // This is a comment
            @15
            D=A
            @LOOP
            (LOOP)
            0;JMP

            // empty line below
            
        "#;

        let result = parse_assembly(input);

        let expected = vec![
            AssemblyCommand::AInstruction("15".to_string()),
            AssemblyCommand::CInstruction("D=A".to_string()),
            AssemblyCommand::AInstruction("LOOP".to_string()),
            AssemblyCommand::Label("LOOP".to_string()),
            AssemblyCommand::CInstruction("0;JMP".to_string()),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic(expected = "Invalid assembly instruction")]
    fn test_parse_lines_invalid_input_panics() {
        let input = "JUMP"; // does not contain '=' or ';'

        parse_assembly(input);
    }

    #[test]
    fn test_assemble_a_instruction() {
        let input = "21";
        let bin = assemble_a_instruction(input);

        assert_eq!(bin, format!("{}{}{}{}", "0000", "0000", "0001", "0101"));
    }

    #[test]
    fn test_dest_equals_comp() {
        let result = assemble_c_instruction("D=A");
        assert_eq!(result, "1110110000010000"); // a=0, comp=A, dest=D, jump=null
    }

    #[test]
    fn test_comp_semicolon_jump() {
        let result = assemble_c_instruction("0;JMP");
        assert_eq!(result, "1110101010000111"); // a=0, comp=0, dest=null, jump=JMP
    }

    #[test]
    fn test_dest_equals_comp_jump() {
        let result = assemble_c_instruction("D=A+1;JGE");
        assert_eq!(result, format!("111{}{}{}{}", "0", "110111", "010", "011"));
    }

    #[test]
    fn test_full_m_form() {
        let result = assemble_c_instruction("AD=D+M;JNE");
        assert_eq!(result, format!("111{}{}{}{}", "1", "000010", "110", "101"));
    }

    #[test]
    fn test_comp_with_m() {
        let result = assemble_c_instruction("D=M");
        assert_eq!(result, format!("111{}{}{}{}", "1", "110000", "010", "000"));
    }

    #[test]
    #[should_panic(expected = "Invalid comp field")]
    fn test_invalid_comp_panics() {
        assemble_c_instruction("D=FOO");
    }

    #[test]
    #[should_panic(expected = "Invalid dest field")]
    fn test_invalid_dest_panics() {
        assemble_c_instruction("X=A");
    }

    #[test]
    #[should_panic(expected = "Invalid jump field")]
    fn test_invalid_jump_panics() {
        assemble_c_instruction("D=A;FLY");
    }


}
