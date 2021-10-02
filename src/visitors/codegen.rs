use anyhow::Result;

use crate::visitors::CodegenVisitorTrait;
use crate::codegen::Codegen;
use crate::ast::*;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;

use std::any::Any;

pub struct CodegenVisitor<'ctx> {
    codegen: Codegen<'ctx>
}

impl<'ctx> CodegenVisitor<'ctx> {
    pub fn new(module: Module<'ctx>, builder: Builder<'ctx> ) -> Self {
        let codegen = Codegen::new(module, builder);

        Self {
            codegen
        }
    }
}

impl<'ctx2> CodegenVisitorTrait for CodegenVisitor<'ctx2> {
    fn get_name(&self) -> String {
        "CodegenVisitor".to_string()
    }

    fn write_bitcode(&self, name: &str) -> Result<bool> {
        self.codegen.write_bitcode(name)?;
        Ok(true)
    }

    fn get_ir(&self) -> Result<Option<String>> {
        Ok(Some(self.codegen.get_ir()))
    }

    fn visit_program(&mut self, _program: &Program) -> Result<()> {
        Ok(())
    }

    fn visit_func(&mut self, _func: &Func) -> Result<()> {
        Ok(())
    }

    fn visit_statement(&mut self, _stmt: &Statement) -> Result<()> {
        Ok(())
    }
    
    //fn visit_guard(&mut self, label: &Option<IdTy>, guard: &mut Guard) -> Result<()>;

    fn visit_guard(&mut self, _guard: &Guard) -> Result<()> {
        Ok(())
    }

    fn visit_expr(&mut self, _expr: &Expr) -> Result<()> {
        Ok(())
    }

    fn visit_term(&mut self, _term: &Term) -> Result<()> {
        Ok(())
    }

    fn visit_struct(&mut self, _stru: &Struct) -> Result<()> {
        Ok(())
    }
}