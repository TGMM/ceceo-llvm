use crate::expr_interpreter::EvalResult;

pub struct BinaryOp {
    pub op: char,
    pub func: fn(Vec<EvalResult>),
}

impl BinaryOp {
    pub const fn new(op: char, func: fn(Vec<EvalResult>)) -> BinaryOp {
        BinaryOp { op, func }
    }
}
