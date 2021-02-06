use crate::grammar;
use crate::visitors::CheckIfFunctionCallExistsVisitor;
use insta::assert_snapshot;

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
    compile!("main() return 1; end;");
}

#[test]
fn test_assignment() {
    compile!("main() var a = 1; var b = 2; end;");
}

#[test]
fn test_reassignment() {
    compile!("main() var a = 1; a = 2; end;");
}

#[test]
fn test_addition() {
    compile!("main() return 1 + 2; end;");
}

#[test]
fn test_multiple_addition() {
    compile!("main() return 1 + 2 + 3; end;");
}

#[test]
fn test_multiple_addition_with_vars() {
    compile!("main() var a = 1; var b = 2; var c = 3; return a + b + c; end;");
}

#[test]
fn test_call_when_names_in_order() {
    compile!("f(a) return a; end; main() var a = 1; return f(a); end;");
}

#[test]
fn test_call_when_names_not_in_order() {
    compile!("main() var a = 1; return f(a); end; f(a) return a; end;");
}

#[test]
fn test_one_guard_with_expr() {
    compile!("main() cond 1 -> return 2; break; end; end;");
}

#[test]
fn test_mixed_guards() {
    compile!("main() cond 1 -> var a = 2; break; -> return 3; break; end; end;");
}