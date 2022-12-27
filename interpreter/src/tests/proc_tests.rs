#![cfg(test)]
use crate::{eval_result::EvalResult, expr_interpreter::handle_list};
use parser::{ast::Atom, parse_ceceo};
#[cfg(test)]
fn get_program_result(program: &str) -> EvalResult {
    let parsed_ceceo = parse_ceceo(program).unwrap();
    let expr = parsed_ceceo.first().unwrap();
    let result = handle_list(expr);
    return result;
}

#[test]
fn all_numeric_procs_work() {
    let program = "(+ 10 10 (* 5 2) (/ 40 2) (* 25 2))";
    let result = get_program_result(program);
    assert_eq!(result, EvalResult::Atom(Atom::Num(100)))
}

#[test]
fn int_sum_returns_zero_if_no_args() {
    let program = "(+)";
    let result = get_program_result(program);
    assert_eq!(result, EvalResult::Atom(Atom::Num(0)))
}

#[test]
fn int_mult_returns_one_if_no_args() {
    let program = "(*)";
    let result = get_program_result(program);
    assert_eq!(result, EvalResult::Atom(Atom::Num(1)))
}

#[test]
fn string_append_works() {
    let program = "(string-append \"Hello \" \"World!\")";
    let result = get_program_result(program);
    assert_eq!(
        result,
        EvalResult::Atom(Atom::Str("Hello World!".to_string()))
    )
}

#[test]
fn and_works() {
    let program = "(and #t 5)";
    let result = get_program_result(program);
    assert_eq!(result, EvalResult::Atom(Atom::Num(5)))
}

#[test]
fn and_returns_true_if_no_args() {
    let program = "(and)";
    let result = get_program_result(program);
    assert_eq!(result, EvalResult::Atom(Atom::Bool(true)))
}

#[test]
fn or_works() {
    let program = "(or #f 5)";
    let result = get_program_result(program);
    assert_eq!(result, EvalResult::Atom(Atom::Num(5)))
}

#[test]
fn or_returns_true_if_no_args() {
    let program = "(or)";
    let result = get_program_result(program);
    assert_eq!(result, EvalResult::Atom(Atom::Bool(false)))
}

#[test]
fn and_works_with_lists() {
    let program = "(and #t (+ 3 2) (or #t #f) (* 10 10))";
    let result = get_program_result(program);
    assert_eq!(result, EvalResult::Atom(Atom::Num(100)))
}

#[test]
fn if_then_branch_works_with_true() {
    let program = "(if #t (+ 2 3) (+ 10 5))";
    let result = get_program_result(program);
    assert_eq!(result, EvalResult::Atom(Atom::Num(5)))
}

#[test]
fn if_then_branch_works_with_any_value() {
    let program = "(if (* 3 (+ 2 5)) (+ 2 3) (+ 10 5))";
    let result = get_program_result(program);
    assert_eq!(result, EvalResult::Atom(Atom::Num(5)))
}

#[test]
fn if_else_branch_works() {
    let program = "(if #f (+ 2 3) (+ 10 5))";
    let result = get_program_result(program);
    assert_eq!(result, EvalResult::Atom(Atom::Num(15)))
}

// TODO: Test console output
#[test]
fn display_works() {
    let program = "(display (+ (- 6 2) [* 3 1] {/ 9 3}))";
    let result = get_program_result(program);
    assert_eq!(
        result,
        EvalResult::QuoteAtom(Atom::Symbol("<void>".to_string()))
    )
}

#[test]
fn not_works_with_false() {
    let program = "(not #f)";
    let result = get_program_result(program);
    assert_eq!(result, EvalResult::Atom(Atom::Bool(true)))
}

#[test]
fn not_works_with_true() {
    let program = "(not #t)";
    let result = get_program_result(program);
    assert_eq!(result, EvalResult::Atom(Atom::Bool(false)))
}

#[test]
fn not_works_with_any_value() {
    let program = "(not (+ 2 3))";
    let result = get_program_result(program);
    assert_eq!(result, EvalResult::Atom(Atom::Bool(false)))
}

#[test]
fn cond_only() {
    let program = "(cond)";
    let result = get_program_result(program);
    assert_eq!(
        result,
        EvalResult::QuoteAtom(Atom::Symbol("<void>".to_string()))
    )
}

#[test]
fn cond_only_else() {
    let program = "(cond [else 5])";
    let result = get_program_result(program);
    assert_eq!(result, EvalResult::Atom(Atom::Num(5)))
}
