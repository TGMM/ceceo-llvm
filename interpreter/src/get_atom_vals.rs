use std::mem::Discriminant;

use parser::ast::Atom;

pub trait GetAtomValues<T> {
    fn get_atom_vals(&self, disc: Discriminant<Atom>) -> Result<Vec<&T>, &'static str>;
}

const DIFFERENT_DISC_ERR: &str = "Different discriminants found";

impl GetAtomValues<i32> for &[Atom] {
    fn get_atom_vals(&self, disc: Discriminant<Atom>) -> Result<Vec<&i32>, &'static str> {
        let mut atom_vals = vec![];
        for atom in self.iter() {
            let atom_disc = std::mem::discriminant(atom);
            if atom_disc != disc {
                return Err(DIFFERENT_DISC_ERR);
            }

            match atom {
                Atom::Num(num) => atom_vals.push(num),
                _ => unimplemented!(),
            }
        }

        return Ok(atom_vals);
    }
}

impl GetAtomValues<String> for &[Atom] {
    fn get_atom_vals(&self, disc: Discriminant<Atom>) -> Result<Vec<&String>, &'static str> {
        let mut atom_vals = vec![];
        for atom in self.iter() {
            let atom_disc = std::mem::discriminant(atom);
            if atom_disc != disc {
                return Err(DIFFERENT_DISC_ERR);
            }

            match atom {
                Atom::Symbol(sym) => atom_vals.push(sym),
                Atom::Str(str) => atom_vals.push(str),
                _ => unimplemented!(),
            }
        }

        return Ok(atom_vals);
    }
}
