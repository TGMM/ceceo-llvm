use ceceo_llvm_parser::ast::Atom;

pub struct BinaryOp {
    pub op: char,
    pub func: fn(Vec<Atom>),
}

impl BinaryOp {
    pub const fn new(op: char, func: fn(Vec<Atom>)) -> BinaryOp {
        BinaryOp { op, func }
    }
}
