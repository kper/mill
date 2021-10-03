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
use inkwell::context::Context;

pub trait CodegenVisitorTrait {
    fn write_bitcode(&self, _name: &str) -> Result<bool> {
        Ok(false)
    }

    fn get_ir(&self) -> Result<Option<String>> {
        Ok(None) 
    }

    fn get_name(&self) -> String;

    fn visit_program<'a>(&mut self, program: &Program, codegen: &'a mut Codegen<'a>, context: &'a Context) -> Result<()>;
    fn visit_func<'a>(&mut self, func: &Func, codegen: &'a mut Codegen<'a>, context: &'a Context) -> Result<()>;
    fn visit_statement<'a>(&mut self, stmt: & Statement, codegen: &'a mut Codegen<'a>, context: &'a Context) -> Result<()>;
    //fn visit_guard(& self, label: &Option<IdTy>, guard: & Guard) -> Result<()>;
    fn visit_guard<'a>(&mut self, guard: & Guard, codegen: &'a mut Codegen<'a>, context: &'a Context) -> Result<()>;
    fn visit_expr<'a>(&mut self, expr: &Expr, codegen: &'a mut Codegen<'a>, context: &'a Context) -> Result<()>;
    fn visit_term<'a>(&mut self, term: &Term, codegen: &'a mut Codegen<'a>, context: &'a Context) -> Result<()>;
    fn visit_struct<'a>(&mut self, stru: &Struct, codegen: &'a mut Codegen<'a>, context: &'a Context) -> Result<()>;
}


pub trait Visitor {
    fn get_name(&self) -> String;

    fn visit_program(&mut self, program: &Program) -> Result<()>;
    fn visit_func(&mut self, func: &Func) -> Result<()>;
    fn visit_statement(&mut self, stmt: &Statement) -> Result<()>;
    //fn visit_guard(& self, label: &Option<IdTy>, guard: & Guard) -> Result<()>;
    fn visit_guard(&mut self, guard: &Guard) -> Result<()>;
    fn visit_expr(&mut self, expr: &Expr) -> Result<()>;
    fn visit_term(&mut self, term: &Term) -> Result<()>;
    fn visit_struct(&mut self, stru: &Struct) -> Result<()>;
}

