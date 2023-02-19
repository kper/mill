use crate::visitors::*;

pub struct PrintVisitor;

impl Visitor for PrintVisitor {
    fn get_name(&self) -> String {
        "PrintVisitor".to_string()
    }

    fn visit_program(&mut self, _program: &Program) -> Result<()> {
        println!("Visiting program");
        Ok(())
    }

    fn visit_func(&mut self, _func: &Func) -> Result<()> {
        println!("Visiting func");
        Ok(())
    }

    fn visit_statement(&mut self, _stmt: &Statement) -> Result<()> {
        println!("Visiting statement");
        Ok(())
    }

    fn visit_expr(&mut self, _expr: &Expr) -> Result<()> {
        println!("Visiting expr");
        Ok(())
    }

    fn visit_term(&mut self, _term: &Term) -> Result<()> {
        println!("Visiting term");
        Ok(())
    }

    fn visit_struct(&mut self, _stru: &Struct) -> Result<()> {
        println!("Visiting struct");
        Ok(())
    }
}
