#![allow(clippy::needless_return)]

pub mod ast;
pub mod lexer;

use ast::Node;
use lalrpop_util::{lalrpop_mod, ParseError};
use lexer::{Lexer, LexicalError, Tok};

lalrpop_mod!(pub ceceo);

pub fn parse_ceceo(
    input: &str,
) -> Result<Vec<Vec<Node>>, ParseError<usize, Tok<'_>, LexicalError>> {
    let lexer = Lexer::new(input);
    let ep = ceceo::ProgramParser::new();

    ep.parse(input, lexer)
}

#[test]
fn basic_expr_test() {
    let input = "(atom 10 \"string\" + - * /)";
    let lexer = Lexer::new(input);
    let ep = ceceo::ProgramParser::new();

    let parsed_expr = ep.parse(input, lexer).unwrap();
    print!("{parsed_expr:?}");
}

#[test]
fn recursive_expr_test() {
    let input = "(atom atom (atom 10 \"string\" + - * /))";
    let lexer = Lexer::new(input);
    let ep = ceceo::ProgramParser::new();

    let parsed_expr = ep.parse(input, lexer).unwrap();
    print!("{parsed_expr:?}");
}

#[test]
fn multiple_expr_test() {
    let input = "(atom atom (atom 10 \"string\" + - * /)) (atom atom (atom 10 \"string\" + - * /))";
    let lexer = Lexer::new(input);
    let ep = ceceo::ProgramParser::new();

    let parsed_expr = ep.parse(input, lexer).unwrap();
    print!("{parsed_expr:?}");
}

#[test]
fn simple_program_test() {
    let input = "(auto i 0) (auto j 1) (auto k 2)";
    let lexer = Lexer::new(input);
    let ep = ceceo::ProgramParser::new();

    let x: Vec<char> = input.chars().skip(57).collect();
    println!("{x:?}");
    let parsed_expr = ep.parse(input, lexer).unwrap();
    print!("{parsed_expr:?}");
}

#[test]
fn real_program_test() {
    let input = "
    (auto i 0)
    (while (< i 30)
      (prog
        (cond ((&& (! (% i 3)) (! (% i 5))) (print \"FizzBuzz\"))
              ((! (% i 3)) (print \"Fizz\"))
              ((! (% i 5)) (print \"Buzz\"))
              (print i))
        (set i (+ i 1))))";
    let lexer = Lexer::new(input);
    let ep = ceceo::ProgramParser::new();

    match ep.parse(input, lexer) {
        Ok(parsed_expr) => println!("{parsed_expr:?}"),
        Err(err) => {
            println!("{err:?}");
            panic!();
        }
    }
}

#[test]
fn quote_list_test() {
    let input = "('(1 2 3) '(\"string\") '(symbol))";
    let lexer = Lexer::new(input);
    let ep = ceceo::ProgramParser::new();

    let parsed_expr = ep.parse(input, lexer).unwrap();
    print!("{parsed_expr:?}");
}

#[test]
fn quote_atom_test() {
    let input = "('1 'symbol '\"string\" '+)";
    let lexer = Lexer::new(input);
    let ep = ceceo::ProgramParser::new();

    let parsed_expr = ep.parse(input, lexer).unwrap();
    print!("{parsed_expr:?}");
}
