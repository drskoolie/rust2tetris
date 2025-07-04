use std::collections::HashMap;

pub fn comp_table() -> HashMap<&'static str, (&'static str, &'static str)> {
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

pub fn dest_table() -> HashMap<&'static str, &'static str> {
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

pub fn jump_table() -> HashMap<&'static str, &'static str> {
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

pub struct SymbolTable {
    table: HashMap<String, u16>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut table = HashMap::new();

        // Predefined symbols
        table.insert("SP".to_string(), 0);
        table.insert("LCL".to_string(), 1);
        table.insert("ARG".to_string(), 2);
        table.insert("THIS".to_string(), 3);
        table.insert("THAT".to_string(), 4);
        for i in 0..16 {
            table.insert(format!("R{}", i), i);
        }
        table.insert("SCREEN".to_string(), 16384);
        table.insert("KBD".to_string(), 24576);

        SymbolTable { table }
    }

    pub fn add_entry(&mut self, symbol: &str, address: u16) {
        self.table.insert(symbol.to_string(), address);
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.table.contains_key(symbol)
    }

    pub fn get_address(&self, symbol: &str) -> Option<u16> {
        self.table.get(symbol).copied()
    }
}


fn comp_mnemonic(a: u16, comp: u16) -> &'static str {
    match (a, comp) {
        (0, 0b101010) => "0",
        (0, 0b111111) => "1",
        (0, 0b111010) => "-1",
        (0, 0b001100) => "D",
        (0, 0b110000) => "A",
        (1, 0b110000) => "M",
        (0, 0b001101) => "!D",
        (0, 0b110001) => "!A",
        (1, 0b110001) => "!M",
        (0, 0b001111) => "-D",
        (0, 0b110011) => "-A",
        (1, 0b110011) => "-M",
        (0, 0b011111) => "D+1",
        (0, 0b110111) => "A+1",
        (1, 0b110111) => "M+1",
        (0, 0b001110) => "D-1",
        (0, 0b110010) => "A-1",
        (1, 0b110010) => "M-1",
        (0, 0b000010) => "D+A",
        (1, 0b000010) => "D+M",
        (0, 0b010011) => "D-A",
        (1, 0b010011) => "D-M",
        (0, 0b000111) => "A-D",
        (1, 0b000111) => "M-D",
        (0, 0b000000) => "D&A",
        (1, 0b000000) => "D&M",
        (0, 0b010101) => "D|A",
        (1, 0b010101) => "D|M",
        _ => "???",
    }
}

fn dest_mnemonic(dest: u16) -> &'static str {
    match dest {
        0b000 => "",
        0b001 => "M",
        0b010 => "D",
        0b011 => "MD",
        0b100 => "A",
        0b101 => "AM",
        0b110 => "AD",
        0b111 => "AMD",
        _ => "???",
    }
}

fn jump_mnemonic(jump: u16) -> &'static str {
    match jump {
        0b000 => "",
        0b001 => "JGT",
        0b010 => "JEQ",
        0b011 => "JGE",
        0b100 => "JLT",
        0b101 => "JNE",
        0b110 => "JLE",
        0b111 => "JMP",
        _ => "???",
    }
}

fn decode_a_instruction(instruction: u16) -> String {
    format!("@{}", instruction)
}

fn decode_c_instruction(instruction: u16) -> String {
    let a = (instruction >> 12) & 0b1;
    let comp = (instruction >> 6) & 0b111111;
    let dest = (instruction >> 3) & 0b111;
    let jump = instruction & 0b111;

    let comp_str = comp_mnemonic(a, comp);
    let dest_str = dest_mnemonic(dest);
    let jump_str = jump_mnemonic(jump);

    match (dest_str, jump_str) {
        ("", "") => format!("{}", comp_str),
        ("", j) => format!("{};{}", comp_str, j),
        (d, "") => format!("{}={}", d, comp_str),
        (d, j) => format!("{}={};{}", d, comp_str, j),
    }
}

pub fn decode_instruction(instruction: u16) -> String {
    if instruction & 0x8000 == 0 {
        decode_a_instruction(instruction)
    } else {
        decode_c_instruction(instruction)
    }
}
