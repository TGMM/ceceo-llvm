pub mod ast;
pub mod lexer;

use lalrpop_util::lalrpop_mod;

#[cfg(test)]
use lexer::Lexer;

lalrpop_mod!(pub ceceo);

fn main() {
    println!("Hello, world!");
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
