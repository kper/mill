use crate::ast::*;
use anyhow::Result;

mod print_visitor;
mod check_if_function_call_exists;
mod codegen;

pub use crate::visitors::print_visitor::*;
pub use crate::visitors::check_if_function_call_exists::*;
pub use crate::visitors::codegen::*;

use crate::codegen::Codegen;

pub trait CodegenVisitorTrait {
    fn write_bitcode(&self, _name: &str) -> Result<bool> {
        Ok(false)
    }

    fn get_ir(&self) -> Result<Option<String>> {
        Ok(None) 
    }

    fn get_name(&self) -> String;

    fn visit_program(&mut self, program: &Program, codegen: &mut Codegen) -> Result<()>;
    fn visit_func(&mut self, func: &Func, codegen: &mut Codegen) -> Result<()>;
    fn visit_statement(&mut self, stmt: & Statement, codegen: &mut Codegen) -> Result<()>;
    fn visit_expr(&mut self, expr: &Expr, codegen: &mut Codegen) -> Result<()>;
    fn visit_term(&mut self, term: &Term, codegen: &mut Codegen) -> Result<()>;
    fn visit_struct(&mut self, stru: &Struct, codegen: &mut Codegen) -> Result<()>;

}


pub trait Visitor {
    fn get_name(&self) -> String;

    fn visit_program(&mut self, program: &Program) -> Result<()>;
    fn visit_func(&mut self, func: &Func) -> Result<()>;
    fn visit_statement(&mut self, stmt: &Statement) -> Result<()>;
    fn visit_guard(&mut self, guard: &Guard) -> Result<()>;
    fn visit_expr(&mut self, expr: &Expr) -> Result<()>;
    fn visit_term(&mut self, term: &Term) -> Result<()>;
    fn visit_struct(&mut self, stru: &Struct) -> Result<()>;
}

