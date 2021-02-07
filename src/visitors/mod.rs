use crate::ast::*;
use crate::symbol_table::SymbolTable;
use anyhow::Result;
use inkwell::values::{BasicValueEnum};
use std::borrow::Cow;

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
