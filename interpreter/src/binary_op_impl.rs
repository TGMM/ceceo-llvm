use std::mem::Discriminant;

use ceceo_llvm_parser::ast::Atom;

use crate::eval_binary_op::EvalBinaryOp;

pub trait BinaryOpImpls<T> {
    fn sum(&self, disc: Discriminant<Atom>) -> T;
    fn mult(&self, disc: Discriminant<Atom>) -> T;
    fn subtract(&self, disc: Discriminant<Atom>) -> T;
}

impl BinaryOpImpls<i32> for Vec<Atom> {
    fn sum(&self, disc: Discriminant<Atom>) -> i32 {
        if self.len() < 1 {
            return 0;
        }

        EvalBinaryOp::<i32>::eval_bop(self, disc, |acc, e| acc + e)
    }

    fn mult(&self, disc: Discriminant<Atom>) -> i32 {
        EvalBinaryOp::<i32>::eval_bop(self, disc, |acc, e| acc * e)
    }

    fn subtract(&self, disc: Discriminant<Atom>) -> i32 {
        if self.len() == 1 && let Atom::Num(n) = self.first().unwrap() {
            return -n;
        }

        EvalBinaryOp::<i32>::eval_bop(self, disc, |acc, e| acc - e)
    }
}

impl BinaryOpImpls<String> for Vec<Atom> {
    fn sum(&self, disc: Discriminant<Atom>) -> String {
        EvalBinaryOp::<String>::eval_bop(self, disc, |acc, e| acc + &e)
    }

    fn mult(&self, _: Discriminant<Atom>) -> String {
        unimplemented!()
    }

    fn subtract(&self, _: Discriminant<Atom>) -> String {
        todo!()
    }
}
