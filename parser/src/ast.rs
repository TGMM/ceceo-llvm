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
    Builtin(Builtin),
}

// TODO: Remove this
#[derive(Debug, PartialEq)]
pub enum Builtin {
    Auto,   // auto
    If,     // if
    While,  // while
    Cond,   // cond
    Prog,   // prog
    Print,  // print
    Sum,    // +
    Sub,    // -
    Less,   // <
    LessEq, // <=
    More,   // >
    Eq,     // =
    Set,    // set
    And,    // and
    Not,    // not
    Mod,    // %
    Null,   // null
    Or,     // |
}
