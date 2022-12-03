pub mod ast;
pub mod lexer;

use ast::Node;
use lalrpop_util::{lalrpop_mod, ParseError};
use lexer::{Lexer, LexicalError, Tok};

lalrpop_mod!(pub ceceo);

pub fn parse_ceceo(input: &str) -> Result<Vec<Node>, ParseError<usize, Tok<'_>, LexicalError>> {
    let lexer = Lexer::new(input);
    let ep = ceceo::ExprParser::new();

    ep.parse(input, lexer)
}

#[test]
fn basic_expr_test() {
    let input = "(atom 10 \"string\" + - * /)";
    let lexer = Lexer::new(input);
    let ep = ceceo::ExprParser::new();

    let parsed_expr = ep.parse(input, lexer).unwrap();
    print!("{:?}", parsed_expr);
}

#[test]
fn recursive_expr_test() {
    let input = "(atom atom (atom 10 \"string\" + - * /))";
    let lexer = Lexer::new(input);
    let ep = ceceo::ExprParser::new();

    let parsed_expr = ep.parse(input, lexer).unwrap();
    print!("{:?}", parsed_expr);
}
