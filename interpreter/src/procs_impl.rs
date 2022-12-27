use crate::{
    eval_iter::{EvalIter, eval_node}, eval_proc::EvalProc,
    generic_procs::GenericProcs, numeric_procs::NumericProcs, string_procs::StringProcs, eval_result::EvalResult,
};
use parser::ast::{Atom, Node};

pub trait ProcImpls<T, U> {
    fn perform_proc(&self, proc_type: U) -> T;
}

impl ProcImpls<i32, NumericProcs> for &[Node] {
    fn perform_proc(&self, proc_type: NumericProcs) -> i32 {
        fn sum(node_slice: &[Node]) -> i32 {
            if node_slice.is_empty() {
                return 0;
            }

            EvalProc::<i32>::eval_proc(&node_slice, |acc, e| acc + e)
        }

        fn mult(node_slice: &[Node]) -> i32 {
            if node_slice.is_empty() {
                return 1;
            }

            EvalProc::<i32>::eval_proc(&node_slice, |acc, e| acc * e)
        }

        fn subtract(node_slice: &[Node]) -> i32 {
            let ret = EvalProc::<i32>::eval_proc(&node_slice, |acc, e| acc - e);
            if node_slice.len() == 1 {
                return -ret;
            }

            return ret;
        }

        fn div(node_slice: &[Node]) -> i32 {
            let ret = EvalProc::<i32>::eval_proc(&node_slice, |acc, e| acc / e);
            if node_slice.len() == 1 {
                return 1 / ret;
            }

            return ret;
        }

        match proc_type {
            NumericProcs::Sum => sum(self),
            NumericProcs::Subtract => subtract(self),
            NumericProcs::Mult => mult(self),
            NumericProcs::Div => div(self),
        }
    }
}

impl ProcImpls<String, StringProcs> for &[Node] {
    fn perform_proc(&self, bop_type: StringProcs) -> String {
        fn append_strings(node_slice: &[Node]) -> String {
            if node_slice.len() == 1 {
                return "".to_string();
            }
            
            EvalProc::<String>::eval_proc(&node_slice, |acc, e| acc + &e)
        }

        match bop_type {
            StringProcs::Append => append_strings(self),
        }
    }
}

impl ProcImpls<EvalResult, GenericProcs> for &[Node] {
    fn perform_proc<'b>(&'b self, proc_type: GenericProcs) -> EvalResult {
        fn and(node_slice: &[Node]) -> EvalResult {
            const DEFAULT: EvalResult = EvalResult::Atom(Atom::Bool(true));
            let mut ret = DEFAULT;

            for n in node_slice.iter_eval() {
                if let EvalResult::Atom(ref a) = n 
                && let Atom::Bool(b) = a 
                && b == &false {
                    return EvalResult::Atom(Atom::Bool(false));
                } else {
                    ret = n;
                }
            }

            return ret;
        }

        fn or(node_slice: &[Node]) -> EvalResult {
            const DEFAULT: EvalResult = EvalResult::Atom(Atom::Bool(false));

            for n in node_slice.iter_eval() {
                if let EvalResult::Atom(ref a) = &n 
                && let Atom::Bool(b) = a 
                && b == &false {
                    continue
                } else {
                    return n;
                }
            }

            return DEFAULT;
        }

        fn if_proc(node_slice: &[Node]) -> EvalResult {
            if node_slice.len() != 3 {
                panic!("Incorrect number of arguments");
            }

            let test_expr = &node_slice[0];
            if let Node::Atom(a) = test_expr 
            && let Atom::Bool(b) = a 
            && b == &false {
                let else_expr = &node_slice[2];
                return eval_node(else_expr);
            } else {
                let then_expr = &node_slice[1];
                return eval_node(then_expr);
            }
        }

        fn display(node_slice: &[Node]) -> EvalResult {
            if node_slice.len() > 1 || node_slice.is_empty() {
                panic!("Incorrect number of arguments");
            }

            let first_eval = eval_node(&node_slice[0]);
            println!("{first_eval}");

            return EvalResult::QuoteAtom(Atom::Symbol("<void>".to_string()));
        }

        match proc_type {
            GenericProcs::And => and(self),
            GenericProcs::Or => or(self),
            GenericProcs::If => if_proc(self),
            GenericProcs::Display => display(self),
        }
    }
}
