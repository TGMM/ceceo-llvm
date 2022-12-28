use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, RwLock},
};

use crate::{
    debug_print,
    eval_iter::eval_node,
    eval_result::EvalResult,
    generic_procs::GenericProcs,
    numeric_procs::NumericProcs,
    procs_impl::{evaluate_and_return_last, ProcImpls},
    string_procs::StringProcs,
    user_proc::UserProc,
};
use parser::ast::{Atom, Node};

// const OPS: [char; 11] = ['+', '-', '*', '<', '>', '%', '\"', '=', '!', '&', '/'];
const INVALID_PROC: &str = "Invalid procedure expression";
pub static DEFINITIONS_MAP: LazyLock<Arc<RwLock<HashMap<String, Node>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));

fn eval_proc<'a>(c: &str, node_args: &'a [Node]) -> EvalResult {
    if let Ok(nproc) = NumericProcs::try_from(c) {
        return EvalResult::Atom(eval_numeric_proc(nproc, node_args));
    } else if let Ok(sproc) = StringProcs::try_from(c) {
        return EvalResult::Atom(eval_string_proc(sproc, node_args));
    } else if let Ok(gproc) = GenericProcs::try_from(c) {
        return eval_generic_proc(gproc, node_args);
    } else {
        let def_map = DEFINITIONS_MAP.read().unwrap();
        let proc_opt = def_map.get(c);

        if let Some(proc_node) = proc_opt
        && let EvalResult::Proc(proc) = eval_node(proc_node) {
            return eval_lambda(proc, node_args);
        }
    }

    panic!("{INVALID_PROC}");
}

fn eval_numeric_proc(proc: NumericProcs, node_args: &[Node]) -> Atom {
    let result = node_args.perform_proc(proc);
    debug_print(&std::format!("{result}"));
    return Atom::Num(result);
}

fn eval_string_proc(proc: StringProcs, node_args: &[Node]) -> Atom {
    let result = node_args.perform_proc(proc);
    debug_print(&std::format!("{result}"));
    return Atom::Str(result);
}

fn eval_generic_proc(proc: GenericProcs, node_args: &[Node]) -> EvalResult {
    let result = node_args.perform_proc(proc);
    debug_print(&std::format!("{result:?}"));
    return result;
}

fn eval_with_proc_atom_and_args(proc_atom: Atom, arg_list: &[Node]) -> EvalResult {
    if let Atom::Symbol(sym) = proc_atom {
        return eval_proc(&sym, arg_list);
    }

    panic!("{INVALID_PROC}");
}

fn eval_lambda(lambda: UserProc, arg_list: &[Node]) -> EvalResult {
    fn replace_symbol_with_arg(
        node: &Node,
        arg_list: &[Node],
        args: &HashMap<String, usize>,
    ) -> Node {
        match node {
            Node::Atom(atom) => match atom {
                Atom::Symbol(sym) => {
                    if let Some(arg_idx) = args.get(sym) {
                        return arg_list[*arg_idx].clone();
                    } else {
                        return node.clone();
                    }
                }
                _ => node.clone(),
            },
            Node::List(list) => Node::List(
                list.iter()
                    .map(|node| replace_symbol_with_arg(node, arg_list, args))
                    .collect::<Vec<Node>>(),
            ),
            _ => node.clone(),
        }
    }

    let expected_arg_count = lambda.get_arity();
    let actual_arg_count = arg_list.len();
    if actual_arg_count != expected_arg_count {
        panic!("Arity mismatch: Expected {expected_arg_count}, got {actual_arg_count} instead");
    }

    let args = lambda.get_args();
    let body = lambda
        .get_body()
        .iter()
        .map(|node| replace_symbol_with_arg(node, arg_list, args))
        .collect::<Vec<Node>>();

    evaluate_and_return_last(&body).unwrap()
}

pub fn eval_list(list: &[Node]) -> EvalResult {
    if list.is_empty() {
        panic!("Missing procedure expression");
    }

    let (procedure, arg_list) = list.split_first().unwrap();
    match procedure {
        Node::Atom(atom) | Node::QuoteAtom(atom) => {
            eval_with_proc_atom_and_args(atom.clone(), arg_list)
        }
        Node::List(list) => match eval_list(list) {
            EvalResult::Atom(atom) => eval_with_proc_atom_and_args(atom, arg_list),
            EvalResult::QuoteList(_) => panic!("{INVALID_PROC}"),
            EvalResult::QuoteAtom(_) => todo!(),
            EvalResult::Proc(proc) => eval_lambda(proc, arg_list),
        },
        Node::QuoteList(_) => panic!("{INVALID_PROC}"),
    }
}

pub fn interpret_ceceo(parsed_ceceo: Vec<Vec<Node>>) {
    for expr in parsed_ceceo {
        eval_list(&expr);
    }
}
