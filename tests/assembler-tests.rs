#[cfg(test)]
mod assembler_tests {
    use revmasm::{
        assembler::{assemble, assemble_one},
        types::bytecodes::bytecodes_to_hex,
    };

    #[test]
    fn test_assemble_one() {
        let instr = "PUSH32 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF";
        let bc = assemble_one(instr.to_string()).unwrap();
        assert_eq!(
            bytecodes_to_hex(&bc),
            "7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
        );
    }
    #[test]
    fn test_assemble() {
        let instrs1 = "PUSH1 0x80 \nPUSH1 0x40\nMSTORE\nPUSH1 0x04\nCALLDATASIZE\nLT\nPUSH1 0x3f\nJUMPI\nPUSH1 0x00\nCALLDATALOAD";
        let bc1 = assemble(instrs1.to_string()).unwrap();
        assert_eq!(
            bytecodes_to_hex(&bc1),
            "608060405260043610603f57600035".to_string()
        );

        let instrs2 = "PUSH2 0xf1f1\nPUSH2 0x0203";
        let bc2 = assemble(instrs2.to_string()).unwrap();
        assert_eq!(bytecodes_to_hex(&bc2), "61f1f1610203".to_string());
    }
}
