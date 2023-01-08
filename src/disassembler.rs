use crate::{
    opcodes::{OPCODE, OPCODE_MAP},
    types::bytecodes::Bytecodes,
};

/// Disassemble the bytecodes
///
/// # Example
///
/// ```
/// use revmasm::{
///     disassembler::disassemble,
///     types::bytecodes::Bytecodes
/// };
///
/// let bc = Bytecodes::from("61F1F1610203".to_string());
/// disassemble(bc);
/// ```
pub fn disassemble(bc: Bytecodes) {
    println!("Rust EVM Disassembler.");
    println!("Bytecodes: {}", bc);
    let kind = match bc.0.len() {
        0..=100 => "struct",
        _ => "hashmap",
    };

    let mut pc = 0;
    while pc < bc.0.len() {
        let opcode = match bc.0.get(pc) {
            Some(opcode) => *opcode,
            None => 0,
        };
        let op_name: String = match kind {
            "struct" => {
                let op = match OPCODE::try_from(opcode) {
                    Ok(op) => op,
                    Err(_) => {
                        println!("{:08x}: `0x{:02x}` unknown opcode", pc, opcode);
                        pc = pc + 1;
                        continue;
                    }
                };
                op.name
            }
            "hashmap" => {
                let name = match OPCODE_MAP.get(&opcode) {
                    Some(name) => *name,
                    None => {
                        println!("{:08x}: `0x{:02x}` unknown opcode", pc, opcode);
                        pc = pc + 1;
                        continue;
                    }
                };

                name.to_string()
            }
            &_ => todo!(),
        };

        if opcode >= 0x60 && opcode <= 0x7f {
            let delta = opcode - 0x60 + 1;
            let start = pc + 1;
            let end = pc + usize::from(delta) + 1;
            println!(
                "{:08x}: {} 0x{}",
                pc,
                op_name,
                hex::encode(bc.0.get(start..end).unwrap())
            );

            pc = pc + usize::from(delta);
        } else {
            println!("{:08x}: {}", pc, op_name);
        }
        pc = pc + 1;
    }
}
