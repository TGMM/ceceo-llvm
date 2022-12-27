#![feature(let_chains)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::needless_return)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::missing_errors_doc)]

mod eval_iter;
mod eval_proc;
pub mod expr_interpreter;
mod generic_procs;
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
    #[clap(action, long)]
    debug: bool,
}

pub fn debug_print(log: &str) {
    unsafe {
        if !SHOULD_DEBUG {
            return;
        }
    }

    println!("DEBUG: {log}");
}

static mut SHOULD_DEBUG: bool = false;

fn main() {
    let args = Arguments::parse();
    unsafe {
        SHOULD_DEBUG = args.debug;
    }
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
