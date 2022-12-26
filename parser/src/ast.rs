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
}
