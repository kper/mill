use crate::ast::*;
use crate::symbol_table::SymbolTable;
use anyhow::Result;
use inkwell::values::{BasicValueEnum};
use std::borrow::Cow;

mod nodes;
mod r#impl;

pub use crate::visitors::r#impl::*;

pub trait CheckIfFunctionCallExistsVisitor {
    fn lookup(functions: &SymbolTable, name: &IdTy) -> bool {
        functions.lookup_symbol(name.get_name())
    }

    fn visit(&self, functions: &SymbolTable) -> Result<bool>;
}

pub trait CodegenVisitor<'ctx> {
    fn visit_program(&mut self, program: &'ctx mut Program) -> Result<()>;
    fn visit_func(&mut self, func: &'ctx mut Func) -> Result<()>;
    fn visit_statement(&mut self, stmt: &mut Statement, function: &IdTy) -> Result<()>;
    fn visit_guard(&mut self, label: &Option<IdTy>, guard: &mut Guard, function: &IdTy) -> Result<()>;
    fn visit_expr(&mut self, expr: &mut Expr, ty: &Option<DataType>) -> Option<Cow<BasicValueEnum<'ctx>>>;
    fn visit_term(&mut self, term: &mut Term) -> Option<Cow<BasicValueEnum<'ctx>>>;
    fn visit_struct(&mut self, mystruct: &Struct) -> Result<()>;
}

pub trait Visitor<'ctx> {
    fn visit_program(&mut self, program: &'ctx mut Program) -> Result<()>;
    fn visit_func(&mut self, func: &'ctx mut Func) -> Result<()>;
    fn visit_statement(&mut self, stmt: &mut Statement) -> Result<()>;
    //fn visit_guard(&mut self, label: &Option<IdTy>, guard: &mut Guard) -> Result<()>;
    fn visit_guard(&mut self, guard: &mut Guard) -> Result<()>;
    fn visit_expr(&mut self, expr: &mut Expr) -> Result<()>;
    fn visit_term(&mut self, term: &mut Term) -> Result<()>;
    fn visit_struct(&mut self, stru: &mut Struct) -> Result<()>;
}

pub trait Traversal {
    fn traverse<'ctx>(&mut self, visitor: &mut Box<dyn Visitor<'ctx>>, program: &'ctx mut Program) -> Result<()>;
}

pub trait AbstractNode<'ctx> {
    fn accept(&'ctx mut self, visitor: &mut impl Visitor<'ctx>);
}