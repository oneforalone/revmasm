use revmasm::{disassembler::disassemble, types::bytecodes::Bytecodes};

fn main() {
    let input = "611a2b";
    let bc = Bytecodes::from(input.to_string());
    disassemble(bc);
}
