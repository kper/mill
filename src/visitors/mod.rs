use crate::ast::*;
use anyhow::Result;
use inkwell::values::{BasicValueEnum};
use std::borrow::Cow;

mod print_visitor;
mod check_if_function_call_exists;
mod codegen;

pub use crate::visitors::print_visitor::*;
pub use crate::visitors::check_if_function_call_exists::*;
pub use crate::visitors::codegen::*;

use crate::codegen::Codegen;

pub trait CodegenVisitorTrait {
    fn get_name(&self) -> String;

    /**
        Only for codegen visitors 
    */
    fn write_bitcode(&self, _name: &str) -> Result<bool> {
        Ok(false)
    }

    /**
        Only for codegen visitors 
    */
    fn get_ir(&self) -> Result<Option<String>> {
        Ok(None) 
    }

    fn visit_program<'ctx>(&mut self, program: &'ctx Program) -> Result<()>;
    fn visit_func<'ctx>(&mut self, func: &'ctx  Func) -> Result<()>;
    fn visit_statement<'ctx>(&mut self, stmt: &'ctx  Statement) -> Result<()>;
    //fn visit_guard(& self, label: &Option<IdTy>, guard: & Guard) -> Result<()>;
    fn visit_guard<'ctx>(&mut self, guard: &'ctx  Guard) -> Result<()>;
    fn visit_expr<'ctx>(&mut self, expr: &'ctx  Expr) -> Result<()>;
    fn visit_term<'ctx>(&mut self, term: &'ctx Term) -> Result<()>;
    fn visit_struct<'ctx>(&mut self, stru: &'ctx Struct) -> Result<()>;

}


pub trait Visitor {
    fn get_name(&self) -> String;

    fn visit_program<'ctx>(&mut self, program: &'ctx Program) -> Result<()>;
    fn visit_func<'ctx>(&mut self, func: &'ctx  Func) -> Result<()>;
    fn visit_statement<'ctx>(&mut self, stmt: &'ctx  Statement) -> Result<()>;
    //fn visit_guard(& self, label: &Option<IdTy>, guard: & Guard) -> Result<()>;
    fn visit_guard<'ctx>(&mut self, guard: &'ctx  Guard) -> Result<()>;
    fn visit_expr<'ctx>(&mut self, expr: &'ctx  Expr) -> Result<()>;
    fn visit_term<'ctx>(&mut self, term: &'ctx Term) -> Result<()>;
    fn visit_struct<'ctx>(&mut self, stru: &'ctx Struct) -> Result<()>;
}

