use crate::{
    eval_iter::EvalIter, eval_proc::EvalProc, expr_interpreter::EvalResult,
    generic_procs::GenericProcs, numeric_procs::NumericProcs, string_procs::StringProcs,
};
use parser::ast::{Atom, Node};

const ZERO_ARGS: &str = "Invalid number of args: 0";

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
            todo!("{node_slice:?}")
        }

        match proc_type {
            GenericProcs::And => and(self),
            GenericProcs::Or => or(self),
            GenericProcs::If => if_proc(self),
        }
    }
}
