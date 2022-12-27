use crate::{
    eval_iter::{EvalIter, eval_node}, eval_proc::EvalProc,
    generic_procs::GenericProcs, numeric_procs::NumericProcs, string_procs::StringProcs, eval_result::EvalResult,
};
use once_cell::sync::Lazy;
use parser::ast::{Atom, Node};

pub trait ProcImpls<T, U> {
    fn perform_proc(&self, proc_type: U) -> T;
}

const INCORRECT_ARG_NUM: &str = "Incorrect number of arguments";

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
        
        fn modulo(node_slice: &[Node]) -> i32 {
            if node_slice.len() != 2 {
                panic!("{INCORRECT_ARG_NUM}");
            }

            let ret = EvalProc::<i32>::eval_proc(&node_slice, |acc, e| acc % e);
            return ret;
        }

        match proc_type {
            NumericProcs::Sum => sum(self),
            NumericProcs::Subtract => subtract(self),
            NumericProcs::Mult => mult(self),
            NumericProcs::Div => div(self),
            NumericProcs::Modulo => modulo(self)
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

static VOID: Lazy<EvalResult> = Lazy::new(|| {
    EvalResult::QuoteAtom(Atom::Symbol("<void>".to_string()))
});

fn node_is_false(node: &Node, should_eval: bool) -> bool {
    if should_eval {
        return eval_result_is_false(&eval_node(node));
    }

    if let Node::Atom(a) = node 
    && let Atom::Bool(b) = a 
    && b == &false {
        return true;
    }

    return false;
}

fn eval_result_is_false(er: &EvalResult) -> bool {
    if let EvalResult::Atom(atom) = er 
    && let Atom::Bool(bool) = atom
    && bool == &false {
        return true;
    } 

    return false;
}

fn evaluate_and_return_last(node_list: &[Node]) -> Option<EvalResult> {
    let mut ret = None;
    for node in node_list {
        ret = Some(eval_node(node));
    }

    return ret;
}

impl ProcImpls<EvalResult, GenericProcs> for &[Node] {
    fn perform_proc<'b>(&'b self, proc_type: GenericProcs) -> EvalResult {
        fn and(node_slice: &[Node]) -> EvalResult {
            const DEFAULT: EvalResult = EvalResult::Atom(Atom::Bool(true));
            let mut ret = DEFAULT;

            for n in node_slice.iter_eval() {
                if eval_result_is_false(&n) {
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
                if eval_result_is_false(&n) {
                    continue
                } else {
                    return n;
                }
            }

            return DEFAULT;
        }

        fn if_proc(node_slice: &[Node]) -> EvalResult {
            if node_slice.len() != 3 {
                panic!("{INCORRECT_ARG_NUM}");
            }

            let test_expr = &node_slice[0];
            if node_is_false(test_expr, false) {
                let else_expr = &node_slice[2];
                return eval_node(else_expr);
            } else {
                let then_expr = &node_slice[1];
                return eval_node(then_expr);
            }
        }

        fn display(node_slice: &[Node]) -> EvalResult {
            if node_slice.len() != 1 {
                panic!("{INCORRECT_ARG_NUM}");
            }

            let first_eval = eval_node(&node_slice[0]);
            println!("{first_eval}");

            return Lazy::force(&VOID).clone();
        }

        fn not(node_slice: &[Node]) -> EvalResult {
            if node_slice.len() != 1 {
                panic!("{INCORRECT_ARG_NUM}");
            }

            let first = &node_slice[0];
            if node_is_false(first, false) {
                return EvalResult::Atom(Atom::Bool(true));
            }

            return EvalResult::Atom(Atom::Bool(false));
        }

        fn cond(node_slice: &[Node]) -> EvalResult {
            fn node_is_else(node: &Node) -> bool {
                if let Node::Atom(atom) = node 
                && let Atom::Symbol(sym) = atom
                && sym == &"else".to_string() {
                    return true;
                }

                return false;
            }

            let node_lists: Vec<&[Node]> = node_slice.iter().map(|n| {
                match n {
                    Node::List(l) if !l.is_empty() => l.as_slice(),
                    _ => panic!("Bad test clause for cond")
                }
            }).collect();

            fn evaluate_conds(node_lists: &[&[Node]]) -> EvalResult {
                let length = node_lists.len();
                for list in node_lists {
                    let test_expr = &list[0];
                    if node_is_else(test_expr) {
                        return match length {
                            1 => evaluate_and_return_last(&list[1..]).expect("Missing expressions in `else' clause"),
                            _ => panic!("Else clause must be last")
                        }
                    }

                    if node_is_false(test_expr, true) {
                        return evaluate_conds(&node_lists[1..]);
                    }

                    return evaluate_and_return_last(list).unwrap_or(Lazy::force(&VOID).clone());
                }

                return Lazy::force(&VOID).clone();
            }

            return evaluate_conds(&node_lists);
        }

        fn test_number(node_slice: &[Node], test_expr: impl Fn(&i32) -> bool) -> EvalResult {
            if node_slice.len() != 1 {
                panic!("{INCORRECT_ARG_NUM}");
            }

            let first = eval_node(&node_slice[0]);
            if let EvalResult::Atom(atom) = first
            && let Atom::Num(num) = atom {
                return EvalResult::Atom(Atom::Bool(test_expr(&num)));
            }

            panic!("Expected number");
        }

        fn is_positive(node_slice: &[Node]) -> EvalResult {
            test_number(node_slice, |num| num > &0)
        }
        
        fn is_zero(node_slice: &[Node]) -> EvalResult {
            test_number(node_slice, |num| num == &0)
        }

        match proc_type {
            GenericProcs::And => and(self),
            GenericProcs::Or => or(self),
            GenericProcs::If => if_proc(self),
            GenericProcs::Display => display(self),
            GenericProcs::Not => not(self),
            GenericProcs::Cond => cond(self),
            GenericProcs::IsPositive => is_positive(self),
            GenericProcs::IsZero => is_zero(self),
        }
    }
}
