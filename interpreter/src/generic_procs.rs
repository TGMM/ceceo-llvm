use bimap::BiHashMap;
use std::sync::LazyLock;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum GenericProcs {
    And,
    Or,
    If,
    Display,
    Not,
    Cond,
    IsPositive,
    IsZero,
    Define,
}

static GENERIC_PROCS_MAP: LazyLock<BiHashMap<GenericProcs, &'static str>> = LazyLock::new(|| {
    BiHashMap::from_iter([
        (GenericProcs::And, "and"),
        (GenericProcs::Or, "or"),
        (GenericProcs::If, "if"),
        (GenericProcs::Display, "display"),
        (GenericProcs::Not, "not"),
        (GenericProcs::Cond, "cond"),
        (GenericProcs::IsPositive, "positive?"),
        (GenericProcs::IsZero, "zero?"),
        (GenericProcs::Define, "define"),
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
