use bytes::{BufMut, BytesMut};

use crate::opcodes::OPCODE;
use crate::types::bytecodes::Bytecodes;

type Error = &'static str;

/// Assemble single EVM OPCODE to bytecodes
///
/// # Example
///
/// ```
/// use revmasm::{
///     assembler::assemble_one,
///     types::bytecodes::bytecodes_to_hex,
/// };
/// let instr = "PUSH2 0x1f2f";
/// let bc = assemble_one(instr.to_string()).unwrap();
/// assert_eq!(bytecodes_to_hex(&bc), "611f2f".to_string());
/// ```
pub fn assemble_one(instr: String) -> Result<Bytecodes, Error> {
    let mut buf = BytesMut::new();
    let instr_str = instr.to_ascii_uppercase();
    let mut ops = instr_str.split_ascii_whitespace();
    let op = ops.next().unwrap();
    let opcode = match OPCODE::try_from(op) {
        Ok(op) => op.opcode,
        Err(_) => todo!(),
    };
    buf.put_u8(opcode);

    // matching pushes opcode
    if op.starts_with("PUSH") {
        let oprand_str = ops.next().unwrap();
        let oprand_hex = oprand_str.strip_prefix("0X").unwrap();
        let oprand = hex::decode(oprand_hex).expect("Decoding oprand_hex Failed");
        buf.put_slice(&oprand);
    }

    Ok(Bytecodes(buf.freeze()))
}

/// Assemble EVM OPCODE to bytecodes
///
/// # Example
///
/// ```
/// use revmasm::{
///     assembler::assemble,
///     types::bytecodes::bytecodes_to_hex,
/// };
/// let instr = "PUSH1 0x80 \nPUSH1 0x40\nMSTORE\nPUSH1 0x04\nCALLDATASIZE\nLT\nPUSH1 0x3f\nJUMPI\nPUSH1 0x00\nCALLDATALOAD";
/// let bc = assemble(instr.to_string()).unwrap();
/// assert_eq!(bytecodes_to_hex(&bc), "608060405260043610603f57600035".to_string());
/// ```
pub fn assemble(instrs: String) -> Result<Bytecodes, Error> {
    println!("Rust EVM Assembler.");
    println!("OPCODEs: \n{}", instrs);
    let mut buf = BytesMut::new();
    for instr in instrs.lines() {
        let bc = assemble_one(instr.to_string()).unwrap();
        buf.put_slice(&bc.0[..]);
    }
    let result = Bytecodes(buf.freeze());
    Ok(result)
}
