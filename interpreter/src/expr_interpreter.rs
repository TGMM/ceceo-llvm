use crate::{procs_impl::ProcImpls, numeric_procs::NumericProcs, string_procs::StringProcs, generic_procs::GenericProcs};
use parser::{
    ast::{Atom, Node},
};

#[derive(Debug, PartialEq)]
pub enum EvalResult<'a> {
    Atom(Atom),
    QuoteList(&'a Vec<Node>),
}

#[derive(Debug, PartialEq)]
pub enum ProcOrSym {
    Symbol,
    Proc(Atom),
}

// const OPS: [char; 11] = ['+', '-', '*', '<', '>', '%', '\"', '=', '!', '&', '/'];

fn handle_procedure(c: &str, ers: Vec<EvalResult>) -> ProcOrSym {
    if let Ok(nproc) = NumericProcs::try_from(c) {
        return ProcOrSym::Proc(handle_numeric_proc(nproc, ers));
    } else if let Ok(sproc) = StringProcs::try_from(c) {
        return ProcOrSym::Proc(handle_string_proc(sproc, ers));
    } else if let Ok(gproc) = GenericProcs::try_from(c) {
        return ProcOrSym::Proc(handle_generic_proc(gproc, ers));
    } else {
        return ProcOrSym::Symbol;
    }
}

fn handle_numeric_proc(proc: NumericProcs, ers: Vec<EvalResult>) -> Atom {
    let atoms = extract_atoms_from_eval_res(ers).expect("");
    let result = atoms.perform_proc(proc);
    println!("{result}");
    return Atom::Num(result);
}

fn handle_string_proc(proc: StringProcs, ers: Vec<EvalResult>) -> Atom {
    let atoms = extract_atoms_from_eval_res(ers).expect("");
    let result = atoms.perform_proc(proc);
    println!("{result}");
    return Atom::Str(result);
}

fn handle_generic_proc(proc: GenericProcs, ers: Vec<EvalResult>) -> Atom {
    let atoms = extract_atoms_from_eval_res(ers).expect("");
    let result = atoms.perform_proc(proc);
    println!("{result:?}");
    return result;
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
    if list.is_empty() {
        panic!("Missing procedure expression");
    }

    let (procedure, arg_list) = list.split_first().unwrap();
    if let Node::List(_) | Node::QuoteList(_) = procedure {
        panic!("Invalid procedure expression");
    }

    if let Node::Atom(atom) = procedure && 
       let Atom::Symbol(sym) = atom
    {
        let eval_args: Vec<EvalResult> = arg_list.iter().map(eval_node).collect();
        if let ProcOrSym::Proc(p) = handle_procedure(sym, eval_args) {
            return EvalResult::Atom(p);
        }

        // TODO: Handle user-defined function
        return EvalResult::Atom(atom.clone());
    } else {
        panic!("Invalid procedure expression");
    }
}

pub fn interpret_ceceo(parsed_ceceo: Vec<Vec<Node>>) {
    for expr in parsed_ceceo {
        handle_list(&expr);
    }
}