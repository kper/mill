use crate::grammar;
use crate::visitors::CheckIfFunctionCallExistsVisitor;
use insta::assert_snapshot;

#[test]
fn test_return() {
    let input = "main() return 1; end;";

    let mut program = grammar::ProgramParser::new().parse(&input).unwrap();
    let functions = program.get_function_names().unwrap();

    program.visit(&functions).unwrap();

    let ir = program.codegen_to_ir().unwrap();

    assert_snapshot!(ir);
}
