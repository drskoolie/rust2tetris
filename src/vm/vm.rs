pub fn push_value(value: u16) -> Vec<String> {
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
    use crate::hardware::cpu::Cpu;
    use crate::parser::assembly::Assembler;

    #[test]
    fn test_translate_push_constant_string() {
        let asm_code = push_value(7);
        let expected = vec![
            "@7", "D=A", "@SP", "A=M", "M=D", "@SP", "M=M+1"
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>();

        assert_eq!(asm_code, expected);
    }
    #[test]
    fn test_translate_push_constant_cpu() {
        let mut cpu = Cpu::new();
        let mut asm = Assembler::new();
        let asm_code = push_value(7);

        asm.assemble_all(&asm_code.join("\n"));
        let no_of_instructions = asm.binaries.len();

        cpu.load_from_string(&asm.binaries.join("\n"));

        assert_eq!(256, cpu.get_data(0));
        for _ in 0..no_of_instructions {
            cpu.clock();
        }

        assert_eq!(257, cpu.get_data(0));
        assert_eq!(7, cpu.get_data(256));

    }

}
