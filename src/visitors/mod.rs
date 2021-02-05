use crate::ast::*;
use crate::codegen::Codegen;
use crate::symbol_table::{Key, LLVMSymbolTable, SymbolTable, Value};
use std::borrow::Cow;
use inkwell::values::{BasicValue, BasicValueEnum, FunctionValue};

type Result<T> = std::result::Result<T, Error>;

pub trait CheckIfFunctionCallExistsVisitor {
    fn lookup(functions: &SymbolTable, name: &IdTy) -> bool {
        functions.lookup_symbol(name)
    }

    fn visit(&self, functions: &SymbolTable) -> Result<bool>;
}

pub trait CodegenVisitor<'ctx> {
    fn visit_program(&mut self, program: &mut Program) -> Result<()>;
    fn visit_func(&mut self, func: &mut Func) -> Result<()>;
    fn visit_statement(
        &mut self,
        stmt: &Statement,
    ) -> Result<()>;
    fn visit_expr(
        &mut self,
        expr: &Expr,
    ) -> Option<Cow<BasicValueEnum<'ctx>>>;
    fn visit_term(
        &mut self,
        term: &Term,
    ) -> Option<Cow<BasicValueEnum<'ctx>>>;
}
