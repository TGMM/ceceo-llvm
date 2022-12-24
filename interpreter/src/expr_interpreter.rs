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

pub enum Bops {
    Sum,
    Subtract,
    Mult,
    Div,
}

impl From<char> for Bops {
    fn from(c: char) -> Self {
        match c {
            '+' => Bops::Sum,
            '-' => Bops::Subtract,
            '*' => Bops::Mult,
            '/' => Bops::Div,
            _ => panic!("Unknown operator")
        }
    }
}

impl Into<char> for Bops {
    fn into(self) -> char {
        match self {
            Bops::Sum => '+',
            Bops::Subtract => '-',
            Bops::Mult => '*',
            Bops::Div => '/',
        }
    }
}

fn handle_bop(bop: Bops, ers: Vec<EvalResult>) -> Atom {
    let atoms = extract_atoms_from_eval_res(ers).expect("");
    let first_atom = atoms.first().unwrap();
    let first_disc = std::mem::discriminant(first_atom);

    match first_atom {
        Atom::Num(_) => {
            let result = atoms.perform_bop(bop, first_disc);
            println!("{}", result);
            return Atom::Num(result);
        }
        Atom::Str(_) => {
            let result = atoms.perform_bop(bop, first_disc); 
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
        let selected_bop = Bops::from(first);
        return EvalResult::Atom(handle_bop(selected_bop, eval_args));
    } else {
        panic!("Invalid procedure expression");
    }
}

pub fn interpret_ceceo(parsed_ceceo: Vec<Vec<Node>>) {
    for expr in parsed_ceceo {
        handle_list(&expr);
    }
}