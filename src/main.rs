use murph::{formatter, parser, Arg};
use std::{env, fs::File, io::Read, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Arg::build(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    let mut file = File::open(config.filename).unwrap_or_else(|err| {
        eprintln!("Error opening file: {}", err);
        process::exit(1);
    });
    let mut bytecode = String::new();
    file.read_to_string(&mut bytecode).unwrap();

    let bytecode = hex::decode(
        bytecode
            .trim()
            .strip_prefix("0x")
            .unwrap_or(bytecode.trim())
            .to_string(),
    )
    .unwrap_or_else(|err| {
        eprintln!("The file must contain only hex code. Error: {}", err);
        process::exit(1);
    });
    let mut parsed = parser::parse(&bytecode).unwrap();

    let huff = formatter::to_huff(&mut parsed);

    println!("{}", &huff);
}
