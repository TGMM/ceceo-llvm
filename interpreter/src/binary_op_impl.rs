use std::mem::Discriminant;

use ceceo_llvm_parser::ast::Atom;

use crate::{eval_binary_op::EvalBinaryOp, expr_interpreter::Bops};

pub trait BinaryOpImpls<T> {
    fn perform_bop(&self, bop_type: Bops, disc: Discriminant<Atom>) -> T;
}

impl BinaryOpImpls<i32> for Vec<Atom> {
    fn perform_bop(&self, bop_type: Bops, disc: Discriminant<Atom>) -> i32 {
        fn sum(va: &Vec<Atom>, disc: Discriminant<Atom>) -> i32 {
            if va.len() < 1 {
                return 0;
            }

            EvalBinaryOp::<i32>::eval_bop(va, disc, |acc, e| acc + e)
        }

        fn mult(va: &Vec<Atom>, disc: Discriminant<Atom>) -> i32 {
            if va.len() < 1 {
                return 1;
            }

            EvalBinaryOp::<i32>::eval_bop(va, disc, |acc, e| acc * e)
        }

        fn subtract(va: &Vec<Atom>, disc: Discriminant<Atom>) -> i32 {
            if va.len() == 1 && let Atom::Num(n) = va.first().unwrap() {
                return -n;
            }

            EvalBinaryOp::<i32>::eval_bop(va, disc, |acc, e| acc - e)
        }

        fn div(va: &Vec<Atom>, disc: Discriminant<Atom>) -> i32 {
            if va.len() == 1 && let Atom::Num(n) = va.first().unwrap() {
                return 1 / n;
            }

            EvalBinaryOp::<i32>::eval_bop(va, disc, |acc, e| acc / e)
        }

        match bop_type {
            Bops::Sum => sum(&self, disc),
            Bops::Subtract => subtract(&self, disc),
            Bops::Mult => mult(&self, disc),
            Bops::Div => div(&self, disc),
        }
    }
}

impl BinaryOpImpls<String> for Vec<Atom> {
    fn perform_bop(&self, bop_type: Bops, disc: Discriminant<Atom>) -> String {
        fn sum(va: &Vec<Atom>, disc: Discriminant<Atom>) -> String {
            EvalBinaryOp::<String>::eval_bop(va, disc, |acc, e| acc + &e)
        }

        match bop_type {
            Bops::Sum => sum(&self, disc),
            Bops::Subtract => unimplemented!(),
            Bops::Mult => unimplemented!(),
            Bops::Div => unimplemented!(),
        }
    }
}
