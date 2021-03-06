use crate::grammar;
use anyhow::anyhow;
use lalrpop_util::ParseError;

macro_rules! extract_user_error {
    ($p:expr) => {{
        match $p {
            ParseError::User { error } => format!("{}", error),
            _ => panic!("wrong error"),
        }
    }};
}

#[test]
fn parse_id() {
    assert!(grammar::IdParser::new().parse("1").is_err());
    assert!(grammar::IdParser::new().parse("a").is_ok());
    assert!(grammar::IdParser::new().parse("a1").is_ok());
    assert!(grammar::IdParser::new().parse("A").is_ok());
    assert!(grammar::IdParser::new().parse("A1").is_ok());
}

#[test]
fn parse_term() {
    assert!(grammar::TermParser::new().parse("1").is_ok());
    assert!(grammar::TermParser::new().parse("11").is_ok());
    assert!(grammar::TermParser::new().parse("(11)").is_ok());
    assert!(grammar::TermParser::new().parse("a").is_ok());
    assert!(grammar::TermParser::new().parse("a1").is_ok());
    assert!(grammar::TermParser::new().parse("A").is_ok());
    assert!(grammar::TermParser::new().parse("A1").is_ok());
}

#[test]
fn parse_expr() {
    assert!(grammar::ExprParser::new().parse("not 1").is_ok());
    assert!(grammar::ExprParser::new().parse("not (1)").is_ok());
    assert!(grammar::ExprParser::new().parse("not a").is_ok());
    assert!(grammar::ExprParser::new().parse("not (a)").is_ok());
    assert!(grammar::ExprParser::new().parse("head (a)").is_ok());
    assert!(grammar::ExprParser::new().parse("tail (a)").is_ok());
    assert!(grammar::ExprParser::new().parse("islist (a)").is_ok());
    assert!(grammar::ExprParser::new().parse("a == a").is_ok());
    assert!(grammar::ExprParser::new().parse("a >= a").is_ok());
    assert!(grammar::ExprParser::new().parse("a == a == a").is_err());
    assert!(grammar::ExprParser::new().parse("a >= a >= a").is_err());
    assert!(grammar::ExprParser::new().parse("a + a").is_ok());
    assert!(grammar::ExprParser::new().parse("a + a + a").is_ok());
    assert!(grammar::ExprParser::new().parse("a + a - a").is_ok());
    assert!(grammar::ExprParser::new().parse("a + a - a or a").is_ok());
    assert!(grammar::ExprParser::new().parse("(a)").is_ok());
    assert!(grammar::ExprParser::new().parse("(1)").is_ok());
}

#[test]
fn parse_statement() {
    // Semicolon is only applied on statements, not singular
    assert!(grammar::StatementParser::new()
        .parse("return head x")
        .is_ok());
    assert!(grammar::StatementParser::new()
        .parse("return not 1")
        .is_ok());
    assert!(grammar::StatementParser::new()
        .parse("return not x")
        .is_ok());
    assert!(grammar::StatementParser::new()
        .parse("return head x")
        .is_ok());
    assert!(grammar::StatementParser::new()
        .parse("return tail x")
        .is_ok());
    assert!(grammar::StatementParser::new()
        .parse("return islist x")
        .is_ok());
    assert!(grammar::StatementParser::new().parse("let x : int = x").is_ok());
    assert!(grammar::StatementParser::new()
        .parse("let x : int= islist x")
        .is_ok());
    assert!(grammar::StatementParser::new().parse("x = x").is_ok());
    assert!(grammar::StatementParser::new()
        .parse("x = islist x")
        .is_ok());
    assert!(grammar::StatementParser::new()
        .parse("id: match _ -> return not a; break; _ -> return not a; break; end")
        .is_ok());
    assert!(grammar::StatementParser::new()
        .parse("x.w = not a")
        .is_ok());
    assert!(grammar::StatementParser::new()
        .parse("x.w = not a.o")
        .is_ok());

}

#[test]
fn parse_statements() {
    assert!(grammar::StatementsParser::new()
        .parse("return not 1; return not 1;")
        .is_ok());
    assert!(grammar::StatementsParser::new()
        .parse("return not 1 return not 1;")
        .is_err());
    assert!(grammar::StatementsParser::new().parse("id: match _ -> return not a; break; _ -> return not a; break; end; match _ -> return not a; continue a; end;").is_ok());
}

#[test]
fn test_assign_errors() {
    assert_eq!(
        extract_user_error!(grammar::FuncdefParser::new()
            .parse("fn x(a : int, b: int, c: int) { let k : int = 1; let k : int = 2; }")
            .unwrap_err()),
        ("Symbol k is already defined")
    );
    assert_eq!(
        extract_user_error!(grammar::FuncdefParser::new()
            .parse("fn x(a : int,b: int,c: int ) { let k : int = 1; h = 2; }")
            .unwrap_err()),
        ("Symbol h is not defined")
    );
}

#[test]
fn parse_func() {
    assert!(grammar::FuncdefParser::new()
        .parse("fn x(a : int ,b : int ,c : int) { return k; }")
        .is_ok());
}

#[test]
fn parse_guard() {
    assert!(grammar::GuardParser::new()
        .parse("_ -> return not a; break")
        .is_ok());
    assert!(grammar::GuardParser::new()
        .parse("_ -> return not a; break a")
        .is_ok());
    assert!(grammar::GuardParser::new()
        .parse("not b -> return not a; break")
        .is_ok());
    assert!(grammar::GuardParser::new()
        .parse("not b -> return not a; break a")
        .is_ok());
}

#[test]
fn parse_cond() {
    assert!(grammar::ConditionalParser::new()
        .parse("match _ -> return not a; break; _ -> return not a; break; end")
        .is_ok());
}

#[test]
fn parse_prog() {
    assert!(grammar::ProgramParser::new()
        .parse("fn x(a: int,b: int,c: int) { return a; } fn test(a: int,b: int,c: int) { return b; }")
        .is_ok());
}

#[test]
fn parse_call() {
    assert!(grammar::TermParser::new().parse("myfunction ()").is_ok());
    assert!(grammar::TermParser::new().parse("myfunction()").is_ok());
    assert!(grammar::TermParser::new().parse("myfunction(a)").is_ok());
    assert!(grammar::TermParser::new()
        .parse("myfunction(a,b,c)")
        .is_ok());
}

#[test]
fn test_function_defined_twice() {
    let parsed = grammar::ProgramParser::new()
        .parse("fn x(a: int,b: int, c: int) { return k; } fn x(a: int,b: int,c: int) { return k; }")
        .unwrap();
    assert_eq!(
        true,
        parsed.check_duplicated_names(),
        "Function name collision was not detected."
    );
}

#[test]
fn test_function_calls_when_defined() {
    use crate::visitors::CheckIfFunctionCallExistsVisitor;

    let parsed = grammar::ProgramParser::new()
        .parse("fn x(a: int,b: int,c: int) { return k(a, b); } fn k(a: int, b: int) { return 1; }")
        .unwrap();

    let functions = parsed.get_function_names().unwrap();
    assert_eq!(parsed.visit(&functions).unwrap(), true);
}

#[test]
fn parse_struct() {
    assert!(grammar::ProgramParser::new()
        .parse("struct test { }")
        .is_ok());
}

#[test]
fn parse_struct_with_func() {
    assert!(grammar::ProgramParser::new()
        .parse("struct test123 { } fn x(a: int,b: int,c: int) { return a; } fn test(a: int,b: int,c: int) { return b; }")
        .is_ok());
}

#[test]
fn parse_struct_with_fields() {
    assert!(grammar::ProgramParser::new()
        .parse("struct test123 { test: int, }")
        .is_ok());
    assert!(grammar::ProgramParser::new()
        .parse("struct CustomStruct { } struct test123 { test: int, test123: i32, test456: CustomStruct }")
        .is_ok());
    assert!(grammar::ProgramParser::new()
        .parse("struct test123 { test: int }")
        .is_ok());

}