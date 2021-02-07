use crate::grammar;
use insta::assert_snapshot;
use crate::visitors::CheckIfFunctionCallExistsVisitor;

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

macro_rules! compile {
    ($input:expr) => {
        init();

        let input = $input;

        let mut program = grammar::ProgramParser::new().parse(&input).unwrap();
        let functions = program.get_function_names().unwrap();

        program.visit(&functions).unwrap();

        let ir = program.codegen_to_ir().unwrap();

        assert_snapshot!(ir);
    };
}

#[test]
fn test_return() {
    compile!("fn main() { return 1; }");
}

#[test]
fn test_assignment() {
    compile!("fn main() { let a = 1; let b = 2; }");
}

#[test]
fn test_reassignment() {
    compile!("fn main() { let a = 1; a = 2; }");
}

#[test]
fn test_addition() {
    compile!("fn main() { return 1 + 2; }");
}

#[test]
fn test_multiple_addition() {
    compile!("fn main() { return 1 + 2 + 3; }");
}

#[test]
fn test_multiple_addition_with_vars() {
    compile!("fn main() { let a = 1; let b = 2; let c = 3; return a + b + c; }");
}

#[test]
fn test_call_when_names_in_order() {
    compile!("fn f(a) { return a; } fn main() { let a = 1; return f(a); }");
}

#[test]
fn test_call_when_names_not_in_order() {
    compile!("fn main() { let a = 1; return f(a); } fn f(a) { return a; }");
}

#[test]
fn test_one_guard_with_expr() {
    compile!("fn main() { match 1 -> return 2; break; end; }");
}

#[test]
fn test_mixed_guards() {
    compile!("fn main() { match 1 -> let a = 2; break; _ -> return 3; break; end; }");
}