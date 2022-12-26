#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::needless_return)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::missing_errors_doc)]

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
