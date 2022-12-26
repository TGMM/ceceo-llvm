use crate::{eval_binary_op::EvalProc, numeric_procs::NumericProcs, string_procs::StringProcs};
use ceceo_llvm_parser::ast::Atom;
use std::mem::Discriminant;

const ZERO_ARGS: &str = "Invalid number of args: 0";

pub trait ProcImpls<T, U> {
    fn perform_proc(&self, bop_type: U) -> T;
}

impl ProcImpls<i32, NumericProcs> for Vec<Atom> {
    fn perform_proc(&self, bop_type: NumericProcs) -> i32 {
        fn sum(va: &Vec<Atom>, disc: Option<Discriminant<Atom>>) -> i32 {
            if va.is_empty() {
                return 0;
            }

            EvalProc::<i32>::eval_bop(va, disc.expect(ZERO_ARGS), |acc, e| acc + e)
        }

        fn mult(va: &Vec<Atom>, disc: Option<Discriminant<Atom>>) -> i32 {
            if va.is_empty() {
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
        let disc = first_atom.map(std::mem::discriminant);

        match bop_type {
            NumericProcs::Sum => sum(self, disc),
            NumericProcs::Subtract => subtract(self, disc),
            NumericProcs::Mult => mult(self, disc),
            NumericProcs::Div => div(self, disc),
        }
    }
}

impl ProcImpls<String, StringProcs> for Vec<Atom> {
    fn perform_proc(&self, bop_type: StringProcs) -> String {
        fn append_strings(va: &Vec<Atom>, disc: Option<Discriminant<Atom>>) -> String {
            EvalProc::<String>::eval_bop(va, disc.expect(ZERO_ARGS), |acc, e| acc + &e)
        }

        let first_atom = self.first();
        let disc = first_atom.map(std::mem::discriminant);

        match bop_type {
            StringProcs::Append => append_strings(self, disc),
        }
    }
}
