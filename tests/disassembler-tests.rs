#[cfg(test)]
mod tests {
    use revmasm::disassembler::disassemble;

    #[test]
    fn test_with_disassemble_default() {
        let input1 = "608060405260043610603f57600035";
        let input2 = "61F1F1610203";
        let kind = "default";

        let decoded1 = hex::decode(input1).expect("Decoding input1 Failed");
        let decoded2 = hex::decode(input2).expect("Decoding input2 Failed");

        disassemble(decoded1, kind);
        disassemble(decoded2, kind);
    }

    #[test]
    fn test_with_disassemble_hashmap() {
        let input1 = "608060405260043610603f57600035";
        let input2 = "61F1F1610203";
        let kind = "hashmap";

        let decoded1 = hex::decode(input1).expect("Decoding input1 Failed");
        let decoded2 = hex::decode(input2).expect("Decoding input2 Failed");

        disassemble(decoded1, kind);
        disassemble(decoded2, kind);
    }
}
