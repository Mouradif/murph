pub mod formatter;
pub mod opcodes;
pub mod parser;
pub mod utils;

pub struct Arg {
    pub filename: String,
}

impl Arg {
    pub fn build(args: &[String]) -> Result<Arg, &'static str> {
        if args.len() != 2 {
            return Err("Usage: murph <bytecode-file>");
        }
        let filename = args[1].clone();
        Ok(Arg { filename })
    }
}
