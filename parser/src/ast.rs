#[derive(Debug, PartialEq)]
pub enum Node {
    Atom(Atom),
    List(Vec<Node>),
    QuoteList(Vec<Node>),
    QuoteAtom(Atom),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Atom {
    Num(i32),
    Symbol(String),
    Str(String),
    Bool(bool),
}

#[must_use]
pub fn parse_hash_symbol(s: &str) -> Atom {
    match s {
        "#true" | "#t" | "#T" => Atom::Bool(true),
        "#false" | "#f" | "#F" => Atom::Bool(false),
        _ => panic!("Unknown hash symbol"),
    }
}
