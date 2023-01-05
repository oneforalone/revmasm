# revmasm

EVM Assembler/Disassembler in rust

## Add revmasm to your repository

```toml
[dependencies]
revmasm = "0.1.2"
```

## Usage

### Assembler

- `assemble_one`

```rust
use revmasm::{
    assembler::assemble_one,
    types::bytecodes::bytecodes_to_hex,
};
let instr = "PUSH32 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF";
let bc = assemble_one(instr.to_string()).unwrap();
assert_eq!(
    bytecodes_to_hex(&bc),
    "7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
);
```

- `assemble`

```rust
use revmasm::{
    assembler::assemble,
    types::bytecodes::bytecodes_to_hex,
};

let instrs1 = "PUSH1 0x80 \nPUSH1 0x40\nMSTORE\nPUSH1 0x04\nCALLDATASIZE\nLT\nPUSH1 0x3f\nJUMPI\nPUSH1 0x00\nCALLDATALOAD";
let bc1 = assemble(instrs1.to_string()).unwrap();
assert_eq!(
    bytecodes_to_hex(&bc1),
    "608060405260043610603f57600035".to_string()
);

let instrs2 = "PUSH2 0xf1f1\nPUSH2 0x0203";
let bc2 = assemble(instrs2.to_string()).unwrap();
assert_eq!(bytecodes_to_hex(&bc2), "61f1f1610203".to_string());
```

### Disassembler

- `disassemble`

```rust
use revmasm::{
    disassembler::disassemble,
    types::bytecodes::Bytecodes
};
let input1 = "608060405260043610603f57600035";
let input2 = "61F1F1610203";

let bc1 = Bytecodes::from(input1.to_string());
let bc2 = Bytecodes::from(input2.to_string());
disassemble(bc1);
disassemble(bc2);
```