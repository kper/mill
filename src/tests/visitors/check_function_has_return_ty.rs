use crate::tests::prelude::*;
use crate::visitors::CheckIfFunctionHasReturnTyVisitor;

#[test]
fn test_function_when_void_but_ret() {
    let input = "fn x(a: int,b: int,c: int) { return a; }";
    let mut passes = vec![Pass::new(
        Box::new(CheckIfFunctionHasReturnTyVisitor::default()),
        Box::new(NormalTraversal),
    )];

    //let result = run_visitor!(input, &mut passes);
    //assert!(result.is_err(), "Expected to throw exception");
}

#[test]
fn test_function_when_not_void_but_ret_void() {
    let input =
        "fn x(a: int,b: int, c: int) -> int { return; }";
    let mut passes = vec![Pass::new(
        Box::new(CheckIfFunctionHasReturnTyVisitor::default()),
        Box::new(NormalTraversal),
    )];

    //let result = run_visitor!(input, &mut passes);
    //assert!(result.is_err(), "Expected to throw exception");
}
