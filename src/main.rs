use revmdis::opcodes::{OPCODE, OPCODE_MAP};
use std::time::Instant;
fn main() {
    let input = "608060405260043610603f57600035";
    // let input = "61F1F1610203";
    let decoded = hex::decode(input).expect("Decoding Failed");

    disassemble(decoded.clone());

    disassemble_v2(decoded.clone());
}

fn disassemble(bytecodes: Vec<u8>) {
    println!("{}", "=".repeat(80));
    let now = Instant::now();
    println!("Disassemble V1:");
    println!("Bytecodes: {}", hex::encode(bytecodes.clone()));
    let mut pc = 0;
    while pc < bytecodes.len() {
        let op = OPCODE::try_from(bytecodes.get(pc).unwrap().to_be()).expect("unknow opcode");
        if op.opcode >= 0x60 && op.opcode <= 0x7f {
            let delta = op.opcode - 0x60 + 1;
            let start = pc + 1;
            let end = pc + usize::from(delta) + 1;
            println!(
                "{:08x}: {} 0x{}",
                pc,
                op.name,
                hex::encode(bytecodes.get(start..end).unwrap())
            );

            pc = pc + usize::from(delta);
        } else {
            println!("{:08x}: {}", pc, op.name);
        }
        pc = pc + 1;
    }

    let duration = now.elapsed().as_micros();
    println!("V1 took {} micros", duration);
    println!("{}", "=".repeat(80));
}

fn disassemble_v2(bytecodes: Vec<u8>) {
    let now = Instant::now();
    println!("Disassemble V2:");
    println!("Bytecodes: {}", hex::encode(bytecodes.clone()));
    let mut pc = 0;
    while pc < bytecodes.len() {
        let opcode = bytecodes.get(pc).unwrap().to_be();
        let op_name = OPCODE_MAP.get(&opcode).unwrap().to_string();
        if opcode >= 0x60 && opcode <= 0x7f {
            let delta = opcode - 0x60 + 1;
            let start = pc + 1;
            let end = pc + usize::from(delta) + 1;
            println!(
                "{:08x}: {} 0x{}",
                pc,
                op_name,
                hex::encode(bytecodes.get(start..end).unwrap())
            );
            pc = pc + usize::from(delta);
        } else {
            println!("{:08x}: {}", pc, op_name);
        }
        pc = pc + 1;
    }

    let duration = now.elapsed().as_micros();
    println!("V2 took {} micros", duration);
    println!("{}", "=".repeat(80));
}
