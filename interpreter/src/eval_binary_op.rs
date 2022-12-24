use std::mem::Discriminant;

use ceceo_llvm_parser::ast::Atom;

use crate::get_atom_vals::GetAtomValues;

pub trait EvalProc<T> {
    fn eval_bop(&self, disc: Discriminant<Atom>, f: impl Fn(T, T) -> T) -> T;
}

impl EvalProc<i32> for Vec<Atom> {
    fn eval_bop(&self, disc: Discriminant<Atom>, reducer: impl Fn(i32, i32) -> i32) -> i32 {
        let vals = GetAtomValues::<i32>::get_atom_vals(self, disc).unwrap();
        let result = vals.iter().map(|v| **v).reduce(reducer).unwrap();
        return result;
    }
}

impl EvalProc<String> for Vec<Atom> {
    fn eval_bop(
        &self,
        disc: Discriminant<Atom>,
        reducer: impl Fn(String, String) -> String,
    ) -> String {
        let vals = GetAtomValues::<String>::get_atom_vals(self, disc).unwrap();
        let result = vals.iter().map(|&v| v.clone()).reduce(reducer).unwrap();
        return result.clone();
    }
}
