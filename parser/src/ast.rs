#[derive(Debug, PartialEq)]
pub enum Node {
    Atom(Atom),
    List(Vec<Node>),
}

#[derive(Debug, PartialEq)]
pub enum Atom {
    Num(i32),
    Symbol(String),
    Str(String),
}
