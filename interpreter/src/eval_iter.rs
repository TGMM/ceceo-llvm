use parser::ast::Node;

use crate::{eval_result::EvalResult, expr_interpreter::handle_list};

pub fn eval_node(node: &Node) -> EvalResult {
    return match node {
        Node::Atom(a) => EvalResult::Atom(a.clone()),
        Node::List(l) => handle_list(l),
        Node::QuoteList(ql) => EvalResult::QuoteList(ql.clone()),
        Node::QuoteAtom(qa) => EvalResult::QuoteAtom(qa.clone()),
    };
}

pub struct NodeIter<'a> {
    internal_node_slice: &'a [Node],
}

impl Iterator for NodeIter<'_> {
    type Item = EvalResult;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.internal_node_slice.first();
        match next {
            Some(next) => {
                self.internal_node_slice = &self.internal_node_slice[1..];

                return Some(eval_node(next));
            }
            None => None,
        }
    }
}

pub trait EvalIter {
    fn iter_eval(&self) -> NodeIter;
}

impl EvalIter for &[Node] {
    fn iter_eval(&self) -> NodeIter {
        return NodeIter {
            internal_node_slice: &self,
        };
    }
}
