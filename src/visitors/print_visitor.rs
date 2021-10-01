use crate::visitors::*;
use crate::ast::*;

pub struct PrintVisitor;

impl Visitor for PrintVisitor{
    fn visit_program<'ctx>(&mut self, program: &'ctx Program) -> Result<()> {
        println!("Visiting program");
        Ok(())
    }

    fn visit_func<'ctx>(&mut self, func: &'ctx Func) -> Result<()> {
        println!("Visiting func");
        Ok(())
    }

    fn visit_statement<'ctx>(&mut self, stmt: &'ctx Statement) -> Result<()> {
        println!("Visiting statement");
        Ok(())
    }
    
    //fn visit_guard(&mut self, label: &Option<IdTy>, guard: &mut Guard) -> Result<()>;

    fn visit_guard<'ctx>(&mut self, guard: &'ctx Guard) -> Result<()> {
        println!("Visiting guard");
        Ok(())
    }

    fn visit_expr<'ctx>(&mut self, expr: &'ctx Expr) -> Result<()> {
        println!("Visiting expr");
        Ok(())
    }

    fn visit_term<'ctx>(&mut self, term: &'ctx Term) -> Result<()> {
        println!("Visiting term");
        Ok(())
    }

    fn visit_struct<'ctx>(&mut self, stru: &'ctx Struct) -> Result<()> {
        println!("Visiting struct");
        Ok(())
    }
}