pub fn translate_push_constant(value: u16) -> Vec<String> {
    vec![
        format!("@{}", value), // Load constant into A
        "D=A".to_string(),    // D = constant
        "@SP".to_string(),    // A = SP location
        "A=M".to_string(),    // A = SP value (actual address)
        "M=D".to_string(),    // RAM[SP] = D = Constant
        "@SP".to_string(),    // A = SP location
        "M=M+1".to_string(),  // Increment SP
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_push_constant() {
        let asm_code = translate_push_constant(7);
        let expected = vec![
            "@7", "D=A", "@SP", "A=M", "M=D", "@SP", "M=M+1"
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>();

        assert_eq!(asm_code, expected);
    }
}
