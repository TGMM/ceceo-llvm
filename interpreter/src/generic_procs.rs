use bimap::BiHashMap;
use once_cell::sync::Lazy;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum GenericProcs {
    And,
    Or,
    If,
    Display,
    Not,
}

static GENERIC_PROCS_MAP: Lazy<BiHashMap<GenericProcs, &'static str>> = Lazy::new(|| {
    BiHashMap::from_iter([
        (GenericProcs::And, "and"),
        (GenericProcs::Or, "or"),
        (GenericProcs::If, "if"),
        (GenericProcs::Display, "display"),
        (GenericProcs::Not, "not"),
    ])
});

impl<'a> TryFrom<&'a str> for GenericProcs {
    type Error = &'static str;
    fn try_from(c: &'a str) -> Result<Self, Self::Error> {
        match GENERIC_PROCS_MAP.get_by_right(c) {
            Some(gp) => Ok(gp.clone()),
            None => Err("Unknown operator"),
        }
    }
}

impl<'a> From<GenericProcs> for &'a str {
    fn from(val: GenericProcs) -> Self {
        GENERIC_PROCS_MAP.get_by_left(&val).unwrap()
    }
}
