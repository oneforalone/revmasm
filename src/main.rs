use revmdis::opcodes::OPCODE;
fn main() {
    let op = OPCODE::MOD;
    let mod_inst = op.get_instruction();
    println!("name: {}, gas: {}", mod_inst.name, mod_inst.gas);
}
