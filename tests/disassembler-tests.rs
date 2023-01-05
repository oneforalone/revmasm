#[cfg(test)]
mod disassembler_tests {
    use revmasm::{disassembler::disassemble, types::bytecodes::Bytecodes};

    #[test]
    fn test_with_disassemble() {
        let input1 = "608060405260043610603f57600035";
        let input2 = "61F1F1610203";

        let bc1 = Bytecodes::from(input1.to_string());
        let bc2 = Bytecodes::from(input2.to_string());
        disassemble(bc1);
        disassemble(bc2);
    }
}
