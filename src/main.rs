use revmasm::disassembler::disassemble;
use std::time::Instant;
fn main() {
    let input = "608060405260043610603f57600035";
    // let input = "61F1F1610203";
    let decoded = hex::decode(input).expect("Decoding Failed");

    let now = Instant::now();
    let kind = "default";
    disassemble(decoded.clone(), kind);
    println!("{} took: {} micros", kind, now.elapsed().as_micros());
}
