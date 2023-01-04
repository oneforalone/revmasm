use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use bytes::{BufMut, BytesMut};

use crate::opcodes::OPCODE;

type Error = &'static str;

#[derive(Clone, Default)]
pub struct Bytecodes(pub bytes::Bytes);

pub fn bytecodes_to_hex(bc: &Bytecodes) -> String {
    hex::encode(bc.0.as_ref())
}

impl Debug for Bytecodes {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Bytecodes(0x{})", bytecodes_to_hex(self))
    }
}

impl Display for Bytecodes {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "0x{}", bytecodes_to_hex(self))
    }
}

pub fn assemble(instrs: String) -> Result<Bytecodes, Error> {
    let mut buf = BytesMut::new();
    for instr in instrs.lines() {
        let bc = assemble_one(instr.to_string()).unwrap();
        buf.put_slice(&bc.0[..]);
    }
    let result = Bytecodes(buf.freeze());
    Ok(result)
}

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
