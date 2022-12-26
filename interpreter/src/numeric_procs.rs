pub enum NumericProcs {
    Sum,
    Subtract,
    Mult,
    Div,
}

impl<'a> TryFrom<&'a str> for NumericProcs {
    type Error = &'static str;
    fn try_from(c: &'a str) -> Result<Self, Self::Error> {
        match c {
            "+" => Ok(NumericProcs::Sum),
            "-" => Ok(NumericProcs::Subtract),
            "*" => Ok(NumericProcs::Mult),
            "/" => Ok(NumericProcs::Div),
            _ => Err("Unknown operator"),
        }
    }
}

impl<'a> From<NumericProcs> for &'a str {
    fn from(val: NumericProcs) -> Self {
        match val {
            NumericProcs::Sum => "+",
            NumericProcs::Subtract => "-",
            NumericProcs::Mult => "*",
            NumericProcs::Div => "/",
        }
    }
}
