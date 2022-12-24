pub enum StringProcs {
    Append,
}

impl TryFrom<&'static str> for StringProcs {
    type Error = &'static str;
    fn try_from(c: &'static str) -> Result<Self, Self::Error> {
        match c {
            "string-append" => Ok(StringProcs::Append),
            _ => Err("Unknown operator"),
        }
    }
}

impl Into<&'static str> for StringProcs {
    fn into(self) -> &'static str {
        match self {
            StringProcs::Append => "string-append",
        }
    }
}
