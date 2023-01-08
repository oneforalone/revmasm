use clap::{arg, Parser};
use revmasm::{
    assembler::assemble,
    {disassembler::disassemble, types::bytecodes::Bytecodes},
};

#[derive(Parser, Debug)]
#[command(author="Yuqi Liu <yuqi.lyle@outlook.com>", version = "0.1.2", about, long_about=None)]
struct ASMArgs {
    /// Assemble EVM OPCODEs to bytecodes
    #[arg(short, long)]
    assemble: Option<String>,
    /// Disassemble EVM bytecodes to OPCODEs
    #[arg(short, long)]
    disassemble: Option<String>,
    // TODO:
    // /// Read from file
    // #[arg(short, long)]
    // file: Option<String>,
    // /// Save result to file
    // #[arg(short, long)]
    // output: Option<String>,
}

fn main() {
    let args = ASMArgs::parse();

    if args.assemble.is_some() {
        let input = args.assemble.unwrap();
        let input = input.replace("\\n", "\n");
        let bc = match assemble(input) {
            Ok(bytecodes) => bytecodes,
            Err(_) => todo!(),
        };
        println!("Bytecodes: {}\n", bc);
    }
    if args.disassemble.is_some() {
        let bc = Bytecodes::from(args.disassemble.unwrap());
        disassemble(bc);
    }
}
