#[cfg(test)]
mod tests {
    use revmasm::opcodes::OPCODE;

    #[test]
    fn test_try_from_u8() -> Result<(), String> {
        for i in 0..=0xff {
            match OPCODE::try_from(i) {
                Ok(_) => return Ok(()),
                Err(err) => return Err(String::from(err)),
            }
        }
        Ok(())
    }

    #[test]
    fn test_try_from_str() -> Result<(), String> {
        let op_names = ["STOP", "PUSH", "PUSH1", "STORE", "END"];
        for name in op_names {
            match OPCODE::try_from(name) {
                Ok(_) => return Ok(()),
                Err(err) => return Err(String::from(err)),
            }
        }

        Ok(())
    }
}
