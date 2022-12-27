use crate::eval_iter::eval_node;
use parser::ast::{Atom, Node};
use std::fmt::Display;

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
