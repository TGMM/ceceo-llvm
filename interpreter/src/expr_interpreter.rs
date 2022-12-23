use crate::binary_op::BinaryOp;
use crate::get_atom_vals::GetAtomValues;
use ceceo_llvm_parser::{
    ast::{Atom, Node},
};

#[derive(Debug, PartialEq)]
pub enum EvalResult<'a> {
    Atom(&'a Atom),
    QuoteList(&'a Vec<Node>),
}

// const OPS: [char; 11] = ['+', '-', '*', '<', '>', '%', '\"', '=', '!', '&', '/'];
const BIN_OPS: [BinaryOp; 1] = [BinaryOp::new('+', handle_sum)];

fn get_op(c: char) -> Option<&'static BinaryOp> {
    BIN_OPS.iter().find(|bop| bop.op == c)
}

fn handle_sum(ers: Vec<EvalResult>) {
    let atoms = extract_atoms_from_eval_res(ers).expect("");
    let first_atom = *atoms.first().unwrap();
    let first_disc = std::mem::discriminant(first_atom);

    match first_atom {
        Atom::Num(_) => {
            let vals = GetAtomValues::<i32>::get_atom_vals(&atoms, first_disc).unwrap();
            println!(
                "{}",
                vals.iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(" + ")
            );
            println!("{}", vals.iter().map(|v| *v).sum::<i32>());
        }
        Atom::Str(_) => {
            let vals = GetAtomValues::<String>::get_atom_vals(&atoms, first_disc).unwrap();
            println!("{}", vals.iter().map(|v| &*(**v)).collect::<Vec<&str>>().join(""));
        }
        Atom::Symbol(_) => unimplemented!(),
    }
}

fn extract_atoms_from_eval_res(ers: Vec<EvalResult>) -> Result<Vec<&Atom>, ()> {
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
        Node::Atom(a) => EvalResult::Atom(a),
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
       let Some(first) = sym.chars().nth(0) &&
       let Some(bop) = get_op(first)
    {
        println!("Found OP {}", bop.op);
        let eval_arg: Vec<EvalResult> = arg_list.iter().map(eval_node).collect();
        (bop.func)(eval_arg);

        return EvalResult::Atom(&Atom::Num(10));
    } else {
        panic!("Invalid procedure expression");
    }
}

pub fn interpret_ceceo(parsed_ceceo: Vec<Vec<Node>>) {
    for expr in parsed_ceceo {
        handle_list(&expr);
    }
}