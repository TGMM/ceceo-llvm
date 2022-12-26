use ceceo_llvm_parser::parse_ceceo;
use clap::Parser;
use std::fs;

#[derive(Parser, Default, Debug)]
struct Arguments {
    file_name: String,
}

fn main() {
    let args = Arguments::parse();
    match fs::read_to_string(args.file_name) {
        Ok(contents) => {
            let parsed_ceceo = parse_ceceo(&contents).unwrap();
            println!("{parsed_ceceo:?}");
        }
        Err(err) => {
            println!("{err}");
        }
    }
}
