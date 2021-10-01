use crate::grammar;
use insta::assert_snapshot;
use crate::visitors::CheckIfFunctionCallExistsVisitor;
use crate::runner::Runner;
use crate::visitors::*;
use crate::pass::Pass;
use crate::traversal::NormalTraversal;

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

macro_rules! compile {
    ($input:expr) => {
        init();

        let input = $input;

        let mut program = grammar::ProgramParser::new().parse(&input).unwrap();

        let mut runner = Runner;
        runner.run_visitors(vec![
            Pass::new(Box::new(CheckIfFunctionCallExistsVisitor::default()), Box::new(NormalTraversal))
        ], &mut program).expect("Running visitor failed");

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
    compile!("fn main() { let a : int = 1; let b : int = 2; }");
}

#[test]
fn test_reassignment() {
    compile!("fn main() { let a : int = 1; a  = 2; }");
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
    compile!("fn main() { let a : int = 1; let b : int = 2; let c : int = 3; return a + b + c; }");
}

#[test]
fn test_call_when_names_in_order() {
    compile!("fn f(a: int) { return a; } fn main() { let a : int = 1; return f(a); }");
}

#[test]
fn test_call_when_names_not_in_order() {
    compile!("fn main() { let a : int = 1; return f(a); } fn f(a:int) { return a; }");
}

#[test]
fn test_one_guard_with_expr() {
    compile!("fn main() { match 1 -> return 2; break; end; }");
}

#[test]
fn test_mixed_guards() {
    compile!("fn main() { match 1 -> let a : int = 2; break; _ -> return 3; break; end; }");
}

#[test]
fn test_struct() {
    compile!("struct test { a: int }");
}

#[test]
fn test_alloc_struct() {
    compile!("struct test { a: int, b: int } fn main() { let a = new test; }");
}

#[test]
fn test_field_access() {
    compile!("struct t { a: int, b: int } fn main() { let o = new t; let k : int = o.a; return k; }");
}

#[test]
fn test_field_assign() {
    compile!("struct t { a: int, b: int } fn main() { let o = new t; o.a = 10; let k : int = o.a; return k; }");
}