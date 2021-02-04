#[macro_use]
extern crate lalrpop_util;

use std::env;
use std::fs::File;
use std::io::Read;

mod ast;

lalrpop_mod!(pub grammar);

fn main() {
    let arguments : Vec<String> = env::args().collect();

    let mut content = String::new();

    for file in arguments.into_iter().skip(1) {
        let mut file_content = String::new();
        let mut fs = File::open(file).expect("Cannot find file");

        fs.read_to_string(&mut file_content).expect("Cannot read into string");
        content.push_str(&file_content);
    }

    //println!("{}", content);

    let ast = grammar::ProgramParser::new().parse(&content).unwrap();

    println!("{:#?}", ast);

    if ast.check_duplicated_names() {
        panic!("Function defined multiple times");
    }
}

#[cfg(test)]
mod tests {
    use crate::grammar;

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
        
        assert!(grammar::StatementParser::new().parse("return not 1").is_ok());
        assert!(grammar::StatementParser::new().parse("return not x").is_ok());
        assert!(grammar::StatementParser::new().parse("return head x").is_ok());
        assert!(grammar::StatementParser::new().parse("return tail x").is_ok());
        assert!(grammar::StatementParser::new().parse("return islist x").is_ok());
        assert!(grammar::StatementParser::new().parse("var x = x").is_ok());
        assert!(grammar::StatementParser::new().parse("var x = islist x").is_ok());
        assert!(grammar::StatementParser::new().parse("x = x").is_ok());
        assert!(grammar::StatementParser::new().parse("x = islist x").is_ok());
    }

    #[test]
    fn parse_statements() {
        assert!(grammar::StatementsParser::new().parse("return not 1; return not 1;").is_ok());
        assert!(grammar::StatementsParser::new().parse("return not 1 return not 1;").is_err());
    }

    #[test]
    fn parse_func() {
        assert!(grammar::FuncdefParser::new().parse("x(a,b,c) return k; end").is_ok());
        assert!(grammar::FuncdefParser::new().parse("x(a,b,c) return k end").is_ok());
    }

    #[test]
    fn parse_prog() {
        assert!(grammar::ProgramParser::new().parse("x(a,b,c) return k; end; x(a,b,c) return k; end;").is_ok());
    }

    #[test]
    fn parse_call() {
        assert!(grammar::TermParser::new().parse("myfunction ()").is_ok());
        assert!(grammar::TermParser::new().parse("myfunction()").is_ok());
        assert!(grammar::TermParser::new().parse("myfunction(a)").is_ok());
        assert!(grammar::TermParser::new().parse("myfunction(a,b,c)").is_ok());
    }

    #[test]
    fn test_function_defined_twice() {
        let parsed = grammar::ProgramParser::new().parse("x(a,b,c) return k; end; x(a,b,c) return k; end;").unwrap();
        assert_eq!(true, parsed.check_duplicated_names(), "Function name collision was not detected.");
    }


}
