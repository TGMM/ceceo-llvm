use crate::{eval_binary_op::EvalBinaryOp, numeric_bops::NumericBops};
use ceceo_llvm_parser::ast::Atom;
use std::mem::Discriminant;

const ZERO_ARGS: &str = "Invalid number of args: 0";

pub trait BinaryOpImpls<T> {
    fn perform_bop(&self, bop_type: NumericBops) -> T;
}

impl BinaryOpImpls<i32> for Vec<Atom> {
    fn perform_bop(&self, bop_type: NumericBops) -> i32 {
        fn sum(va: &Vec<Atom>, disc: Option<Discriminant<Atom>>) -> i32 {
            if va.len() < 1 {
                return 0;
            }

            EvalBinaryOp::<i32>::eval_bop(va, disc.expect(ZERO_ARGS), |acc, e| acc + e)
        }

        fn mult(va: &Vec<Atom>, disc: Option<Discriminant<Atom>>) -> i32 {
            if va.len() < 1 {
                return 1;
            }

            EvalBinaryOp::<i32>::eval_bop(va, disc.expect(ZERO_ARGS), |acc, e| acc * e)
        }

        fn subtract(va: &Vec<Atom>, disc: Option<Discriminant<Atom>>) -> i32 {
            if va.len() == 1 && let Atom::Num(n) = va.first().unwrap() {
                return -n;
            }

            EvalBinaryOp::<i32>::eval_bop(va, disc.expect(ZERO_ARGS), |acc, e| acc - e)
        }

        fn div(va: &Vec<Atom>, disc: Option<Discriminant<Atom>>) -> i32 {
            if va.len() == 1 && let Atom::Num(n) = va.first().unwrap() {
                return 1 / n;
            }

            EvalBinaryOp::<i32>::eval_bop(va, disc.expect(ZERO_ARGS), |acc, e| acc / e)
        }

        let first_atom = self.first();
        let disc = first_atom.map(|a| std::mem::discriminant(a));

        match bop_type {
            NumericBops::Sum => sum(&self, disc),
            NumericBops::Subtract => subtract(&self, disc),
            NumericBops::Mult => mult(&self, disc),
            NumericBops::Div => div(&self, disc),
        }
    }
}
