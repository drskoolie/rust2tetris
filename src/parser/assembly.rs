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

}
