#![cfg(test)]
use crate::expr_interpreter::{handle_list, EvalResult};
use ceceo_llvm_parser::{ast::Atom, parse_ceceo};

#[test]
fn all_numeric_procs_work() {
    let program = "(+ 10 10 (* 5 2) (/ 40 2) (* 25 2))";

    let parsed_ceceo = parse_ceceo(program).unwrap();
    let expr = parsed_ceceo.first().unwrap();
    let result = handle_list(&expr);

    assert_eq!(result, EvalResult::Atom(Atom::Num(100)))
}

#[test]
fn int_sum_returns_zero_if_no_args() {
    let program = "(+)";

    let parsed_ceceo = parse_ceceo(program).unwrap();
    let expr = parsed_ceceo.first().unwrap();
    let result = handle_list(&expr);

    assert_eq!(result, EvalResult::Atom(Atom::Num(0)))
}

#[test]
fn int_mult_returns_one_if_no_args() {
    let program = "(*)";

    let parsed_ceceo = parse_ceceo(program).unwrap();
    let expr = parsed_ceceo.first().unwrap();
    let result = handle_list(&expr);

    assert_eq!(result, EvalResult::Atom(Atom::Num(1)))
}

#[test]
fn string_append_works() {
    let program = "(string-append \"Hello \" \"World!\")";

    let parsed_ceceo = parse_ceceo(program).unwrap();
    let expr = parsed_ceceo.first().unwrap();
    let result = handle_list(&expr);

    assert_eq!(
        result,
        EvalResult::Atom(Atom::Str("Hello World!".to_string()))
    )
}
