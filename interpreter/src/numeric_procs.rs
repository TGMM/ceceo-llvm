use bimap::BiHashMap;
use once_cell::sync::Lazy;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum NumericProcs {
    Sum,
    Subtract,
    Mult,
    Div,
}

static NUMERIC_PROCS_MAP: Lazy<BiHashMap<NumericProcs, &'static str>> = Lazy::new(|| {
    BiHashMap::from_iter([
        (NumericProcs::Sum, "+"),
        (NumericProcs::Subtract, "-"),
        (NumericProcs::Mult, "*"),
        (NumericProcs::Div, "/"),
    ])
});

impl<'a> TryFrom<&'a str> for NumericProcs {
    type Error = &'static str;
    fn try_from(c: &'a str) -> Result<Self, Self::Error> {
        match NUMERIC_PROCS_MAP.get_by_right(c) {
            Some(gp) => Ok(gp.clone()),
            None => Err("Unknown operator"),
        }
    }
}

impl<'a> From<NumericProcs> for &'a str {
    fn from(val: NumericProcs) -> Self {
        NUMERIC_PROCS_MAP.get_by_left(&val).unwrap()
    }
}
