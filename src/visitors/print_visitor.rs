use crate::visitors::*;
use std::any::Any;

pub struct PrintVisitor;

impl Visitor for PrintVisitor{
    fn get_name(&self) -> String {
        "PrintVisitor".to_string()
    }

    fn visit_program<'ctx>(&mut self, _program: &'ctx Program) -> Result<()> {
        println!("Visiting program");
        Ok(())
    }

    fn visit_func<'ctx>(&mut self, _func: &'ctx Func) -> Result<()> {
        println!("Visiting func");
        Ok(())
    }

    fn visit_statement<'ctx>(&mut self, _stmt: &'ctx Statement) -> Result<()> {
        println!("Visiting statement");
        Ok(())
    }
    
    //fn visit_guard(&mut self, label: &Option<IdTy>, guard: &mut Guard) -> Result<()>;

    fn visit_guard<'ctx>(&mut self, _guard: &'ctx Guard) -> Result<()> {
        println!("Visiting guard");
        Ok(())
    }

    fn visit_expr<'ctx>(&mut self, _expr: &'ctx Expr) -> Result<()> {
        println!("Visiting expr");
        Ok(())
    }

    fn visit_term<'ctx>(&mut self, _term: &'ctx Term) -> Result<()> {
        println!("Visiting term");
        Ok(())
    }

    fn visit_struct<'ctx>(&mut self, _stru: &'ctx Struct) -> Result<()> {
        println!("Visiting struct");
        Ok(())
    }
}