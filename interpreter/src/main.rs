#![feature(let_chains)]

mod binary_op_impl;
mod eval_binary_op;
pub mod expr_interpreter;
mod get_atom_vals;
mod numeric_bops;
mod tests;

use ceceo_llvm_parser::parse_ceceo;
use clap::Parser;
use expr_interpreter::interpret_ceceo;
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
            interpret_ceceo(parsed_ceceo);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}
