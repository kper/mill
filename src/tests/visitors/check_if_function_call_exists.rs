use crate::tests::prelude::*;
use crate::visitors::CheckIfFunctionCallExistsVisitor;

#[test]
fn test_function_calls_when_defined() {
    let input = "fn x(a: int,b: int,c: int) { return k(a, b); } fn k(a: int, b: int) { return 1; }";
    let mut passes = vec![Pass::new(
        Box::new(CheckIfFunctionCallExistsVisitor::default()),
        Box::new(NormalTraversal),
    )];

    //let _result = run_visitor!(input, &mut passes).expect("should work");
}

#[test]
fn test_function_defined_twice() {
    let input =
        "fn x(a: int,b: int, c: int) { return k; } fn x(a: int,b: int,c: int) { return k; }";
    let mut passes = vec![Pass::new(
        Box::new(CheckIfFunctionCallExistsVisitor::default()),
        Box::new(NormalTraversal),
    )];

    //let result = run_visitor!(input, &mut passes);
    //assert!(result.is_err(), "Expected to throw exception");
}
