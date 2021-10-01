use crate::tests::prelude::*;

#[test]
fn test_function_calls_when_defined() {
    use crate::visitors::CheckIfFunctionCallExistsVisitor;

    let input = "fn x(a: int,b: int,c: int) { return k(a, b); } fn k(a: int, b: int) { return 1; }";
    let passes = vec![
        Pass::new(Box::new(CheckIfFunctionCallExistsVisitor::default()), Box::new(NormalTraversal))
    ];

    let _result = run_visitor!(input, passes).expect("should work");
}