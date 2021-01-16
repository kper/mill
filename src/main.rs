#[macro_use]
extern crate lalrpop_util;

mod ast;

lalrpop_mod!(pub grammar);

fn main() {
    println!("Hello, world!");
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



}
