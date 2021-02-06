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
    assert!(grammar::StatementParser::new().parse("var x = x").is_ok());
    assert!(grammar::StatementParser::new()
        .parse("var x = islist x")
        .is_ok());
    assert!(grammar::StatementParser::new().parse("x = x").is_ok());
    assert!(grammar::StatementParser::new()
        .parse("x = islist x")
        .is_ok());
    assert!(grammar::StatementParser::new()
        .parse("id: cond -> return not a; break; -> return not a; break; end")
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
    assert!(grammar::StatementsParser::new().parse("id: cond -> return not a; break; -> return not a; break; end; cond -> return not a; continue a; end;").is_ok());
}

#[test]
fn test_assign_errors() {
    assert_eq!(
        extract_user_error!(grammar::FuncdefParser::new()
            .parse("x(a,b,c) var k = 1; var k = 2; end")
            .unwrap_err()),
        ("Symbol k is already defined")
    );
    assert_eq!(
        extract_user_error!(grammar::FuncdefParser::new()
            .parse("x(a,b,c) var k = 1; h = 2; end")
            .unwrap_err()),
        ("Symbol h is not defined")
    );
}

#[test]
fn parse_func() {
    assert!(grammar::FuncdefParser::new()
        .parse("x(a,b,c) return k; end")
        .is_ok());
}

#[test]
fn parse_guard() {
    assert!(grammar::GuardParser::new()
        .parse("-> return not a; break")
        .is_ok());
    assert!(grammar::GuardParser::new()
        .parse("-> return not a; break a")
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
        .parse("cond -> return not a; break; -> return not a; break; end")
        .is_ok());
}

#[test]
fn parse_prog() {
    assert!(grammar::ProgramParser::new()
        .parse("x(a,b,c) return k; end; x(a,b,c) return k; end;")
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
        .parse("x(a,b,c) return k; end; x(a,b,c) return k; end;")
        .unwrap();
    assert_eq!(
        true,
        parsed.check_duplicated_names(),
        "Function name collision was not detected."
    );
}

/*
#[test]
fn test_function_calls_when_not_defined() {
    use crate::visitors::CheckIfFunctionCallExistsVisitor;

    let parsed = grammar::ProgramParser::new()
        .parse("x(a,b,c) return k(a, b); end;")
        .unwrap();

    let functions = parsed.get_function_names().unwrap();
    assert_eq!(
        extract_user_error!(parsed.visit(&functions).unwrap_err()),
        ("Function k is not defined")
    );
}*/

#[test]
fn test_function_calls_when_defined() {
    use crate::visitors::CheckIfFunctionCallExistsVisitor;

    let parsed = grammar::ProgramParser::new()
        .parse("x(a,b,c) return k(a, b); end; k(a, b) return 1; end;")
        .unwrap();

    let functions = parsed.get_function_names().unwrap();
    assert_eq!(parsed.visit(&functions).unwrap(), true);
}
