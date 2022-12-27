#![cfg(test)]
use crate::expr_interpreter::{handle_list, EvalResult};
use parser::{ast::Atom, parse_ceceo};

#[test]
fn all_numeric_procs_work() {
    let program = "(+ 10 10 (* 5 2) (/ 40 2) (* 25 2))";

    let parsed_ceceo = parse_ceceo(program).unwrap();
    let expr = parsed_ceceo.first().unwrap();
    let result = handle_list(expr);

    assert_eq!(result, EvalResult::Atom(Atom::Num(100)))
}

#[test]
fn int_sum_returns_zero_if_no_args() {
    let program = "(+)";

    let parsed_ceceo = parse_ceceo(program).unwrap();
    let expr = parsed_ceceo.first().unwrap();
    let result = handle_list(expr);

    assert_eq!(result, EvalResult::Atom(Atom::Num(0)))
}

#[test]
fn int_mult_returns_one_if_no_args() {
    let program = "(*)";

    let parsed_ceceo = parse_ceceo(program).unwrap();
    let expr = parsed_ceceo.first().unwrap();
    let result = handle_list(expr);

    assert_eq!(result, EvalResult::Atom(Atom::Num(1)))
}

#[test]
fn string_append_works() {
    let program = "(string-append \"Hello \" \"World!\")";

    let parsed_ceceo = parse_ceceo(program).unwrap();
    let expr = parsed_ceceo.first().unwrap();
    let result = handle_list(expr);

    assert_eq!(
        result,
        EvalResult::Atom(Atom::Str("Hello World!".to_string()))
    )
}

#[test]
fn and_works() {
    let program = "(and #t 5)";

    let parsed_ceceo = parse_ceceo(program).unwrap();
    let expr = parsed_ceceo.first().unwrap();
    let result = handle_list(expr);

    assert_eq!(result, EvalResult::Atom(Atom::Num(5)))
}

#[test]
fn and_returns_true_if_no_args() {
    let program = "(and)";

    let parsed_ceceo = parse_ceceo(program).unwrap();
    let expr = parsed_ceceo.first().unwrap();
    let result = handle_list(expr);

    assert_eq!(result, EvalResult::Atom(Atom::Bool(true)))
}

#[test]
fn or_works() {
    let program = "(or #f 5)";

    let parsed_ceceo = parse_ceceo(program).unwrap();
    let expr = parsed_ceceo.first().unwrap();
    let result = handle_list(expr);

    assert_eq!(result, EvalResult::Atom(Atom::Num(5)))
}

#[test]
fn or_returns_true_if_no_args() {
    let program = "(or)";

    let parsed_ceceo = parse_ceceo(program).unwrap();
    let expr = parsed_ceceo.first().unwrap();
    let result = handle_list(expr);

    assert_eq!(result, EvalResult::Atom(Atom::Bool(false)))
}

#[test]
fn and_works_with_lists() {
    let program = "(and #t (+ 3 2) (or #t #f) (* 10 10))";

    let parsed_ceceo = parse_ceceo(program).unwrap();
    let expr = parsed_ceceo.first().unwrap();
    let result = handle_list(expr);

    assert_eq!(result, EvalResult::Atom(Atom::Num(100)))
}

#[test]
fn if_then_branch_works_with_true() {
    let program = "(if #t (+ 2 3) (+ 10 5))";
    let parsed_ceceo = parse_ceceo(program).unwrap();
    let expr = parsed_ceceo.first().unwrap();
    let result = handle_list(expr);

    assert_eq!(result, EvalResult::Atom(Atom::Num(5)))
}

#[test]
fn if_then_branch_works_with_any_value() {
    let program = "(if (* 3 (+ 2 5)) (+ 2 3) (+ 10 5))";
    let parsed_ceceo = parse_ceceo(program).unwrap();
    let expr = parsed_ceceo.first().unwrap();
    let result = handle_list(expr);

    assert_eq!(result, EvalResult::Atom(Atom::Num(5)))
}

#[test]
fn if_else_branch_works() {
    let program = "(if #f (+ 2 3) (+ 10 5))";
    let parsed_ceceo = parse_ceceo(program).unwrap();
    let expr = parsed_ceceo.first().unwrap();
    let result = handle_list(expr);

    assert_eq!(result, EvalResult::Atom(Atom::Num(15)))
}
