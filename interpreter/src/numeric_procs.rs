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

impl<'a> Into<&'a str> for NumericProcs {
    fn into(self) -> &'static str {
        match self {
            NumericProcs::Sum => "+",
            NumericProcs::Subtract => "-",
            NumericProcs::Mult => "*",
            NumericProcs::Div => "/",
        }
    }
}
