use crate::{binary_op_impl::BinaryOpImpls, numeric_bops::NumericBops};
use ceceo_llvm_parser::{
    ast::{Atom, Node},
};

#[derive(Debug, PartialEq)]
pub enum EvalResult<'a> {
    Atom(Atom),
    QuoteList(&'a Vec<Node>),
}

// const OPS: [char; 11] = ['+', '-', '*', '<', '>', '%', '\"', '=', '!', '&', '/'];

fn handle_bop(c: char, ers: Vec<EvalResult>) -> Atom {
    if let Ok(nbop) = NumericBops::try_from(c) {
        return handle_numeric_bop(nbop, ers);
    } else {
        unimplemented!();
    }
}

fn handle_numeric_bop(bop: NumericBops, ers: Vec<EvalResult>) -> Atom {
    let atoms = extract_atoms_from_eval_res(ers).expect("");

    let result = atoms.perform_bop(bop);
    println!("{}", result);
    return Atom::Num(result);
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

pub fn handle_list(list: &Vec<Node>) -> EvalResult {
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
        return EvalResult::Atom(handle_bop(first, eval_args));
    } else {
        panic!("Invalid procedure expression");
    }
}

pub fn interpret_ceceo(parsed_ceceo: Vec<Vec<Node>>) {
    for expr in parsed_ceceo {
        handle_list(&expr);
    }
}