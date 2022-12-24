pub enum NumericBops {
    Sum,
    Subtract,
    Mult,
    Div,
}

impl TryFrom<char> for NumericBops {
    type Error = &'static str;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '+' => Ok(NumericBops::Sum),
            '-' => Ok(NumericBops::Subtract),
            '*' => Ok(NumericBops::Mult),
            '/' => Ok(NumericBops::Div),
            _ => Err("Unknown operator"),
        }
    }
}

impl Into<char> for NumericBops {
    fn into(self) -> char {
        match self {
            NumericBops::Sum => '+',
            NumericBops::Subtract => '-',
            NumericBops::Mult => '*',
            NumericBops::Div => '/',
        }
    }
}
