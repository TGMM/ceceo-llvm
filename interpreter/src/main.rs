#![feature(let_chains)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::needless_return)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::missing_errors_doc)]

mod eval_binary_op;
pub mod expr_interpreter;
mod generic_procs;
mod get_atom_vals;
mod numeric_procs;
mod procs_impl;
mod string_procs;
mod tests;

use clap::Parser;
use expr_interpreter::interpret_ceceo;
use parser::parse_ceceo;
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
            println!("{err}");
        }
    }
}
