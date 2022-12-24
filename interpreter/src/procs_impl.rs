use crate::{eval_binary_op::EvalProc, numeric_procs::NumericProcs};
use ceceo_llvm_parser::ast::Atom;
use std::mem::Discriminant;

const ZERO_ARGS: &str = "Invalid number of args: 0";

pub trait ProcImpls<T> {
    fn perform_bop(&self, bop_type: NumericProcs) -> T;
}

impl ProcImpls<i32> for Vec<Atom> {
    fn perform_bop(&self, bop_type: NumericProcs) -> i32 {
        fn sum(va: &Vec<Atom>, disc: Option<Discriminant<Atom>>) -> i32 {
            if va.len() < 1 {
                return 0;
            }

            EvalProc::<i32>::eval_bop(va, disc.expect(ZERO_ARGS), |acc, e| acc + e)
        }

        fn mult(va: &Vec<Atom>, disc: Option<Discriminant<Atom>>) -> i32 {
            if va.len() < 1 {
                return 1;
            }

            EvalProc::<i32>::eval_bop(va, disc.expect(ZERO_ARGS), |acc, e| acc * e)
        }

        fn subtract(va: &Vec<Atom>, disc: Option<Discriminant<Atom>>) -> i32 {
            if va.len() == 1 && let Atom::Num(n) = va.first().unwrap() {
                return -n;
            }

            EvalProc::<i32>::eval_bop(va, disc.expect(ZERO_ARGS), |acc, e| acc - e)
        }

        fn div(va: &Vec<Atom>, disc: Option<Discriminant<Atom>>) -> i32 {
            if va.len() == 1 && let Atom::Num(n) = va.first().unwrap() {
                return 1 / n;
            }

            EvalProc::<i32>::eval_bop(va, disc.expect(ZERO_ARGS), |acc, e| acc / e)
        }

        let first_atom = self.first();
        let disc = first_atom.map(|a| std::mem::discriminant(a));

        match bop_type {
            NumericProcs::Sum => sum(&self, disc),
            NumericProcs::Subtract => subtract(&self, disc),
            NumericProcs::Mult => mult(&self, disc),
            NumericProcs::Div => div(&self, disc),
        }
    }
}
