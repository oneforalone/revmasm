use crate::{
    opcodes::{OPCODE, OPCODE_MAP},
    types::bytecodes::Bytecodes,
};

pub struct Instruction {
    pub pc: usize,
    pub name: String,
    pub data: Option<String>,
}

pub fn disassemble(bc: Bytecodes) -> Vec<Instruction> {
    let kind = determine_kind(&bc);
    let mut pc = 0;
    let mut instructions = Vec::new();

    while pc < bc.0.len() {
        let opcode = get_opcode(&bc, pc);

        match process_opcode(&bc, opcode, &kind, pc) {
            Some((instruction, next_pc)) => {
                instructions.push(instruction);
                pc = next_pc;
            }
            None => {
                pc += 1;
            }
        }
    }

    instructions
}

fn determine_kind(bc: &Bytecodes) -> &'static str {
    match bc.0.len() {
        0..=100 => "struct",
        _ => "hashmap",
    }
}

fn get_opcode(bc: &Bytecodes, pc: usize) -> u8 {
    bc.0.get(pc).copied().unwrap_or(0)
}

fn process_opcode(bc: &Bytecodes, opcode: u8, kind: &str, pc: usize) -> Option<(Instruction, usize)> {
    let op_name = get_opcode_name(opcode, kind)?;

    if is_push(opcode) {
        let (data, next_pc) = extract_push_data(bc, opcode, pc);
        let instruction = Instruction {
            pc,
            name: op_name,
            data: Some(data),
        };
        Some((instruction, next_pc))
    } else {
        let instruction = Instruction {
            pc,
            name: op_name,
            data: None,
        };
        Some((instruction, pc + 1))
    }
}

fn get_opcode_name(opcode: u8, kind: &str) -> Option<String> {
    match kind {
        "struct" => OPCODE::try_from(opcode).ok().map(|op| op.name),
        "hashmap" => OPCODE_MAP.get(&opcode).map(|name| name.to_string()),
        _ => None,
    }
}

/// (0x60-0x7f) means push
fn is_push(opcode: u8) -> bool {
    opcode >= 0x60 && opcode <= 0x7f
}

fn extract_push_data(bc: &Bytecodes, opcode: u8, pc: usize) -> (String, usize) {
    let delta = opcode - 0x60 + 1;
    let start = pc + 1;
    let end = pc + usize::from(delta) + 1;

    let data = match bc.0.get(start..end) {
        Some(bytes) => hex::encode(bytes),
        None => String::new(),
    };

    (data, end)
}