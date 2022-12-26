pub enum GenericProcs {
    And,
    Or,
}

impl<'a> TryFrom<&'a str> for GenericProcs {
    type Error = &'static str;
    fn try_from(c: &'a str) -> Result<Self, Self::Error> {
        match c {
            "and" => Ok(GenericProcs::And),
            "or" => Ok(GenericProcs::Or),
            _ => Err("Unknown operator"),
        }
    }
}

impl<'a> From<GenericProcs> for &'a str {
    fn from(val: GenericProcs) -> Self {
        match val {
            GenericProcs::And => "and",
            GenericProcs::Or => "or",
        }
    }
}
