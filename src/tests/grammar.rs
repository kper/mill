use crate::grammar;
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
    assert!(grammar::ExprParser::new().parse("a == a").is_ok());
    assert!(grammar::ExprParser::new().parse("(a)").is_ok());
    assert!(grammar::ExprParser::new().parse("a").is_ok());
    assert!(grammar::ExprParser::new().parse("(1)").is_ok());
}

#[test]
fn parse_statement() {
    assert!(grammar::StatementParser::new().parse("let x : int = x").is_ok());
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
fn parse_prog() {
    assert!(grammar::ProgramParser::new().parse("fn myfunction () {} fn myfunction2() {}").is_ok());
}

#[test]
fn parse_call() {
    assert!(grammar::FuncdefParser::new().parse("fn myfunction () {}").is_ok());
    assert!(grammar::FuncdefParser::new().parse("fn myfunction() {}").is_ok());
    assert!(grammar::FuncdefParser::new().parse("fn myfunction(a : int) { }").is_ok());
    assert!(grammar::FuncdefParser::new()
        .parse("fn myfunction(a : int ,b : int ,c : int) {}")
        .is_ok());
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
        .parse("struct test123 { } fn x(a: int,b: int,c: int) { } fn test(a: int,b: int,c: int) { }")
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