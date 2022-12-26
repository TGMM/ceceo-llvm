pub enum StringProcs {
    Append,
}

impl<'a> TryFrom<&'a str> for StringProcs {
    type Error = &'static str;
    fn try_from(c: &'a str) -> Result<Self, Self::Error> {
        match c {
            "string-append" => Ok(StringProcs::Append),
            _ => Err("Unknown operator"),
        }
    }
}

impl From<StringProcs> for &'static str {
    fn from(val: StringProcs) -> Self {
        match val {
            StringProcs::Append => "string-append",
        }
    }
}
