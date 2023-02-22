use crate::grammar;
use crate::tests::prelude::*;
use insta::assert_snapshot;

macro_rules! compile {
    ($input:expr) => {
        use llvm_sys::core::*;

        let _ = env_logger::builder().is_test(true).try_init();

        let input = $input;

        // Setup LLVM
        unsafe {
            let context = LLVMContextCreate();
            let module = LLVMModuleCreateWithName(c_str!("main"));
            let builder = LLVMCreateBuilderInContext(context);
            let mut codegen = Codegen::new(context, module, builder);
            let mut runner = Runner;

            // Parse
            let mut program = grammar::ProgramParser::new().parse(&input).unwrap();

            // Run the visitors
            let mut passes = crate::default_passes();
            runner
                .run_visitors(&mut passes, &mut program)
                .expect("Running visitor failed");

            // Codegen and get IR
            runner
                .run_codegen(
                    &mut CodegenVisitor::new(),
                    &mut codegen,
                    CodegenTraversal,
                    &mut program,
                )
                .expect("Codegen failed");

            let ir = crate::utils::LLVMString::new(LLVMPrintModuleToString(module)).to_string();

            assert_snapshot!(ir);
        }
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
    compile!("fn main() { let a : int = 1; a = 2; }");
}

#[test]
fn test_addition() {
    compile!("fn main() { return 1 + 2; }");
}

#[test]
fn test_conditional() {
    compile!("fn main() -> int { if 2 == 2 { return 1; } return 0; }");
}

#[test]
fn test_conditional_with_else() {
    compile!("fn main() -> int { if 2 == 2 { return 1; } else { return 0; } return 2; }");
}

#[test]
fn test_call_when_names_in_order() {
    compile!(
        "fn f(b: int) -> int { return b; } fn main() -> int { let a : int = 1; return f(a); }"
    );
}

#[test]
fn test_call_when_names_not_in_order() {
    compile!("fn main() -> int { let a : int = 1; return f(a); } fn f(a:int) -> int { return a; }");
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
    compile!(
        "struct t { a: int, b: int } fn main() { let o = new t; let k : int = o.a; return k; }"
    );
}

#[test]
fn test_field_assign() {
    compile!("struct t { a: int, b: int } fn main() { let o = new t; o.a = 10; let k : int = o.a; return k; }");
}
