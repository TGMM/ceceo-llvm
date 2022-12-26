use parser::ast::{Atom, Node};

use crate::{eval_iter::EvalIter, expr_interpreter::EvalResult};

pub trait EvalProc<T> {
    fn eval_proc(&self, f: impl Fn(T, T) -> T) -> T;
}

impl EvalProc<i32> for &[Node] {
    fn eval_proc(&self, reducer: impl Fn(i32, i32) -> i32) -> i32 {
        self.iter_eval()
            .map(|er| {
                if let EvalResult::Atom(atom) = er && let Atom::Num(num) = atom {
                return num;
            } else {
                panic!("Incorrect type: Expected number");
            }
            })
            .reduce(reducer)
            .unwrap()
    }
}

impl EvalProc<String> for &[Node] {
    fn eval_proc(&self, reducer: impl Fn(String, String) -> String) -> String {
        self.iter_eval()
            .map(|er| {
                if let EvalResult::Atom(atom) = er && let Atom::Str(st) = atom {
                return st;
            } else {
                panic!("Incorrect type: Expected string");
            }
            })
            .reduce(reducer)
            .unwrap()
    }
}
