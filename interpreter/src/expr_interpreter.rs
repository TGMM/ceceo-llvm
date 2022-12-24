use std::{mem::Discriminant};

use crate::{binary_op_impl::BinaryOpImpls};
use ceceo_llvm_parser::{
    ast::{Atom, Node},
};

#[derive(Debug, PartialEq)]
pub enum EvalResult<'a> {
    Atom(Atom),
    QuoteList(&'a Vec<Node>),
}

// const OPS: [char; 11] = ['+', '-', '*', '<', '>', '%', '\"', '=', '!', '&', '/'];

enum Bops {
    Sum,
    Subtract,
    Mult,
}

type BopFn<T> = fn(&Vec<Atom>, disc: Discriminant<Atom>) -> T;
fn get_bop<T>(bop: Bops) -> BopFn<T> where Vec<Atom>: BinaryOpImpls<T>
{
    return match bop {
        Bops::Sum => <Vec<Atom> as BinaryOpImpls<T>>::sum,
        Bops::Subtract => <Vec<Atom> as BinaryOpImpls<T>>::subtract,
        Bops::Mult => <Vec<Atom> as BinaryOpImpls<T>>::mult,
    };
}

fn handle_bop(bop: Bops, ers: Vec<EvalResult>) -> Atom {
    let atoms = extract_atoms_from_eval_res(ers).expect("");
    let first_atom = atoms.first().unwrap();
    let first_disc = std::mem::discriminant(first_atom);

    match first_atom {
        Atom::Num(_) => {
            let result = (get_bop(bop))(&atoms, first_disc);
            println!("{}", result);
            return Atom::Num(result);
        }
        Atom::Str(_) => {
            let result = (get_bop(bop))(&atoms, first_disc); 
            println!("{}", result);
            return Atom::Str(result);
        }
        _ => unimplemented!(),
    }
}

fn extract_atoms_from_eval_res(ers: Vec<EvalResult>) -> Result<Vec<Atom>, ()> {
    let mut atoms = vec![];
    for er in ers {
        match er {
            EvalResult::Atom(a) => {
                atoms.push(a);
            },
            EvalResult::QuoteList(_) => {
                return Err(());
            },
        }
    }

    return Ok(atoms);
}

fn eval_node(node: &Node) -> EvalResult {
    return match node {
        Node::Atom(a) => EvalResult::Atom(a.clone()),
        Node::List(l) => handle_list(l),
        Node::QuoteList(ql) => EvalResult::QuoteList(ql),
        Node::QuoteAtom(_) => panic!("Invalid argument"),
    };
}

fn handle_list(list: &Vec<Node>) -> EvalResult {
    if list.len() < 1 {
        panic!("Missing procedure expression");
    }

    let (procedure, arg_list) = list.split_first().unwrap();
    if let Node::List(_) | Node::QuoteList(_) = procedure {
        panic!("Invalid procedure expression");
    }

    if let Node::Atom(atom) = procedure && 
       let Atom::Symbol(sym) = atom && 
       let Some(first) = sym.chars().nth(0)
    {
        let eval_args: Vec<EvalResult> = arg_list.iter().map(eval_node).collect();
        match first {
            '+' => {
                return EvalResult::Atom(handle_bop(Bops::Sum, eval_args));
            }
            '*' => {
                return EvalResult::Atom(handle_bop(Bops::Mult, eval_args));
            }
            '-' => {
                return EvalResult::Atom(handle_bop(Bops::Subtract, eval_args));
            }
            _ => panic!("Unknown operator")
        }
    } else {
        panic!("Invalid procedure expression");
    }
}

pub fn interpret_ceceo(parsed_ceceo: Vec<Vec<Node>>) {
    for expr in parsed_ceceo {
        handle_list(&expr);
    }
}