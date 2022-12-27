use std::fmt::Display;

use crate::{
    debug_print, eval_iter::eval_node, generic_procs::GenericProcs, numeric_procs::NumericProcs,
    procs_impl::ProcImpls, string_procs::StringProcs,
};
use parser::ast::{Atom, Node};

#[derive(Debug, PartialEq, Clone)]
pub enum EvalResult {
    Atom(Atom),
    QuoteAtom(Atom),
    QuoteList(Vec<Node>),
}

impl Display for EvalResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalResult::Atom(a) | EvalResult::QuoteAtom(a) => match a {
                Atom::Num(n) => write!(f, "{n}"),
                Atom::Symbol(s) => write!(f, "{s}"),
                Atom::Str(str) => write!(f, "{str}"),
                Atom::Bool(b) => write!(f, "{b}"),
            },
            EvalResult::QuoteList(ql) => {
                for node in ql {
                    let node_eval = eval_node(node);
                    return write!(f, "{node_eval}");
                }

                Ok(())
            }
        }
    }
}

// const OPS: [char; 11] = ['+', '-', '*', '<', '>', '%', '\"', '=', '!', '&', '/'];
const INVALID_PROC: &str = "Invalid procedure expression";

fn handle_procedure<'a>(c: &str, node_args: &'a [Node]) -> EvalResult {
    if let Ok(nproc) = NumericProcs::try_from(c) {
        return EvalResult::Atom(handle_numeric_proc(nproc, node_args));
    } else if let Ok(sproc) = StringProcs::try_from(c) {
        return EvalResult::Atom(handle_string_proc(sproc, node_args));
    } else if let Ok(gproc) = GenericProcs::try_from(c) {
        return handle_generic_proc(gproc, node_args);
    } else {
        // TODO: Handle user defined functions
        panic!("{INVALID_PROC}");
    }
}

fn handle_numeric_proc(proc: NumericProcs, node_args: &[Node]) -> Atom {
    let result = node_args.perform_proc(proc);
    debug_print(&std::format!("{result}"));
    return Atom::Num(result);
}

fn handle_string_proc(proc: StringProcs, node_args: &[Node]) -> Atom {
    let result = node_args.perform_proc(proc);
    debug_print(&std::format!("{result}"));
    return Atom::Str(result);
}

fn handle_generic_proc(proc: GenericProcs, node_args: &[Node]) -> EvalResult {
    let result = node_args.perform_proc(proc);
    debug_print(&std::format!("{result:?}"));
    return result;
}

fn handle_proc_atom(proc_atom: Atom, arg_list: &[Node]) -> EvalResult {
    if let Atom::Symbol(sym) = proc_atom {
        return handle_procedure(&sym, arg_list);
    }

    panic!("{INVALID_PROC}");
}

pub fn handle_list(list: &[Node]) -> EvalResult {
    if list.is_empty() {
        panic!("Missing procedure expression");
    }

    let (procedure, arg_list) = list.split_first().unwrap();
    match procedure {
        Node::Atom(atom) | Node::QuoteAtom(atom) => handle_proc_atom(atom.clone(), arg_list),
        Node::List(list) => match handle_list(list) {
            EvalResult::Atom(atom) => handle_proc_atom(atom, arg_list),
            EvalResult::QuoteList(_) => panic!("{INVALID_PROC}"),
            EvalResult::QuoteAtom(_) => todo!(),
        },
        Node::QuoteList(_) => panic!("{INVALID_PROC}"),
    }
}

pub fn interpret_ceceo(parsed_ceceo: Vec<Vec<Node>>) {
    for expr in parsed_ceceo {
        handle_list(&expr);
    }
}
