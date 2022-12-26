use crate::{
    eval_proc::EvalProc, generic_procs::GenericProcs, numeric_procs::NumericProcs,
    string_procs::StringProcs,
};
use parser::ast::Atom;
use std::mem::Discriminant;

const ZERO_ARGS: &str = "Invalid number of args: 0";

pub trait ProcImpls<T, U> {
    fn perform_proc(&self, proc_type: U) -> T;
}

impl ProcImpls<i32, NumericProcs> for Vec<Atom> {
    fn perform_proc(&self, proc_type: NumericProcs) -> i32 {
        fn sum(va: &[Atom], disc: Option<Discriminant<Atom>>) -> i32 {
            if va.is_empty() {
                return 0;
            }

            EvalProc::<i32>::eval_proc(&va, disc.expect(ZERO_ARGS), |acc, e| acc + e)
        }

        fn mult(va: &[Atom], disc: Option<Discriminant<Atom>>) -> i32 {
            if va.is_empty() {
                return 1;
            }

            EvalProc::<i32>::eval_proc(&va, disc.expect(ZERO_ARGS), |acc, e| acc * e)
        }

        fn subtract(va: &[Atom], disc: Option<Discriminant<Atom>>) -> i32 {
            if va.len() == 1 && let Atom::Num(n) = va.first().unwrap() {
                return -n;
            }

            EvalProc::<i32>::eval_proc(&va, disc.expect(ZERO_ARGS), |acc, e| acc - e)
        }

        fn div(va: &[Atom], disc: Option<Discriminant<Atom>>) -> i32 {
            if va.len() == 1 && let Atom::Num(n) = va.first().unwrap() {
                return 1 / n;
            }

            EvalProc::<i32>::eval_proc(&va, disc.expect(ZERO_ARGS), |acc, e| acc / e)
        }

        let first_atom = self.first();
        let disc = first_atom.map(std::mem::discriminant);

        match proc_type {
            NumericProcs::Sum => sum(self, disc),
            NumericProcs::Subtract => subtract(self, disc),
            NumericProcs::Mult => mult(self, disc),
            NumericProcs::Div => div(self, disc),
        }
    }
}

impl ProcImpls<String, StringProcs> for Vec<Atom> {
    fn perform_proc(&self, bop_type: StringProcs) -> String {
        fn append_strings(va: &[Atom], disc: Option<Discriminant<Atom>>) -> String {
            EvalProc::<String>::eval_proc(&va, disc.expect(ZERO_ARGS), |acc, e| acc + &e)
        }

        let first_atom = self.first();
        let disc = first_atom.map(std::mem::discriminant);

        match bop_type {
            StringProcs::Append => append_strings(self, disc),
        }
    }
}

impl ProcImpls<Atom, GenericProcs> for Vec<Atom> {
    fn perform_proc(&self, proc_type: GenericProcs) -> Atom {
        fn and(va: &[Atom]) -> Atom {
            if va.is_empty() {
                return Atom::Bool(true);
            }

            if va.contains(&Atom::Bool(false)) {
                return Atom::Bool(false);
            }

            return va.last().unwrap().to_owned();
        }

        fn or(va: &[Atom]) -> Atom {
            if va.is_empty() {
                return Atom::Bool(false);
            }

            let first = va.first().unwrap();
            if first != &Atom::Bool(false) {
                return first.to_owned();
            }

            let va_without_first = &va[1..];
            return or(va_without_first);
        }

        fn if_proc(va: &[Atom]) -> Atom {
            todo!("{va:?}")
        }

        match proc_type {
            GenericProcs::And => and(self),
            GenericProcs::Or => or(self),
            GenericProcs::If => if_proc(self),
        }
    }
}
