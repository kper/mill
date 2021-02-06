use crate::ast::*;
use crate::symbol_table::SymbolTable;
use inkwell::values::BasicValueEnum;
use std::borrow::Cow;
use anyhow::Result;

pub trait CheckIfFunctionCallExistsVisitor {
    fn lookup(functions: &SymbolTable, name: &IdTy) -> bool {
        functions.lookup_symbol(name)
    }

    fn visit(&self, functions: &SymbolTable) -> Result<bool>;
}

pub trait CodegenVisitor<'ctx> {
    fn visit_program(&mut self, program: &'ctx mut Program) -> Result<()>;
    fn visit_func(&mut self, func: &'ctx mut Func) -> Result<()>;
    fn visit_statement(&mut self, stmt: &Statement) -> Result<()>;
    fn visit_expr(&mut self, expr: &Expr) -> Option<Cow<BasicValueEnum<'ctx>>>;
    fn visit_term(&mut self, term: &Term) -> Option<Cow<BasicValueEnum<'ctx>>>;
}
