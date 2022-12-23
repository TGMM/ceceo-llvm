mod binary_op;
mod get_atom_vals;

use binary_op::BinaryOp;
use ceceo_llvm_parser::{
    ast::{Atom, Node},
    parse_ceceo,
};
use clap::Parser;
use get_atom_vals::GetAtomValues;
use std::fs;

#[derive(Parser, Default, Debug)]
struct Arguments {
    file_name: String,
}

const OPS: [char; 11] = ['+', '-', '*', '<', '>', '%', '\"', '=', '!', '&', '/'];
const BIN_OPS: [BinaryOp; 1] = [BinaryOp::new('+', handle_sum)];

fn get_op(c: char) -> Option<&'static BinaryOp> {
    BIN_OPS.iter().find(|bop| bop.op == c)
}

fn handle_sum(atoms: Vec<Atom>) {
    let first_atom = atoms.first().unwrap();
    let first_disc = std::mem::discriminant(first_atom);

    match first_atom {
        Atom::Num(_) => {
            let vals = GetAtomValues::<i32>::get_atom_vals(atoms, first_disc).unwrap();
            println!(
                "{}",
                vals.iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(" + ")
            );
            println!("{}", vals.iter().sum::<i32>());
        }
        Atom::Str(_) => {
            let vals = GetAtomValues::<String>::get_atom_vals(atoms, first_disc).unwrap();
            println!("{}", vals.join(""));
        }
        Atom::Symbol(_) => unimplemented!(),
    }
}

fn handle_atom(atom: Atom) {
    match atom {
        ceceo_llvm_parser::ast::Atom::Num(num) => {
            println!("Found number {}", num)
        }
        ceceo_llvm_parser::ast::Atom::Symbol(sym) => {
            if sym.len() != 1 {
                println!("Found symbol {}", sym);
                return;
            }

            let first = sym.chars().nth(0).unwrap();
            if let Some(bop) = get_op(first) {
                println!("Found OP {}", bop.op);
                (bop.func)(vec![Atom::Num(10), Atom::Num(10)]);
            } else {
                println!("Found symbol {}", sym);
            }
        }
        ceceo_llvm_parser::ast::Atom::Str(str) => {
            println!("Found string \"{}\"", str)
        }
    }
}

fn handle_node(node: Node) {
    match node {
        Node::Atom(atom) => handle_atom(atom),
        Node::List(list) => handle_list(list),
        Node::QuoteList(_) => todo!(),
        Node::QuoteAtom(_) => todo!(),
    }
}

fn handle_list(list: Vec<Node>) {
    for node in list {
        handle_node(node);
    }
}

fn main() {
    let args = Arguments::parse();
    match fs::read_to_string(args.file_name) {
        Ok(contents) => {
            let parsed_ceceo = parse_ceceo(&contents).unwrap();
            for expr in parsed_ceceo {
                handle_list(expr);
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}
