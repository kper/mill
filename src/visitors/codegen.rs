use anyhow::Result;

use crate::Visitor;
use crate::visitors::CodegenVisitorTrait;
use crate::codegen::Codegen;
use crate::ast::*;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;


use inkwell::values::{BasicValue, BasicValueEnum};

use log::debug;

pub struct CodegenVisitor {
}

impl CodegenVisitor {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl<'a> CodegenVisitorTrait<'a> for CodegenVisitor {
    fn get_name(&self) -> String {
        "CodegenVisitor".to_string()
    }

    fn visit_program(&mut self, _program: &Program, codegen: &mut Codegen<'a>) -> Result<()> {
        debug!("{}: running visit_program", self.get_name());
        Ok(())
    }

    fn visit_func(&mut self, func: &Func, codegen: &mut Codegen<'a>) -> Result<()> {
        debug!("{}: running visit_func", self.get_name());

        //let context = codegen.get_context();

        //let builder = codegen.get_mut_builder();
        //let module = codegen.get_mut_module();
        //let context = codegen.get_context();
        //let ftable = codegen.get_mut_function_table();
        //let symbol_table = codegen.get_mut_symtable();

        let context = codegen.context;
        let i64_type = context.i64_type();
        let func_types = vec![i64_type.into(); func.pars.len()]; 
        let fn_type = i64_type.fn_type(&func_types, false);

        let function = codegen.get_mut_module().add_function(&func.id.get_name(), fn_type, None);

        codegen.get_mut_function_table().insert(&func.id.get_name(), function)?;

        /* 
        // Basic block

        let basic = context.append_basic_block(function, func.id.get_name());

        builder.position_at_end(basic);

        for (i, param) in func.pars.iter().enumerate() {
            let value = function.get_nth_param(i as u32).unwrap();
            let ptr = builder.build_alloca(i64_type, param.get_name());

            let _instr = builder.build_store(ptr, value);

            symbol_table.insert(
                param.get_name(),
                (param.clone(), BasicValueEnum::PointerValue(ptr)),
            )?;
            debug!("Allocating functions parameter {}", param);
        }
        */

        Ok(())
    }

    fn visit_statement(&mut self, _stmt: &Statement, codegen: &mut Codegen<'a>) -> Result<()> {
        Ok(())
    }
    
    fn visit_guard(&mut self, _guard: &Guard, codegen: &mut Codegen<'a>) -> Result<()> {
        Ok(())
    }

    fn visit_expr(&mut self, _expr: &Expr, codegen: &mut Codegen<'a>) -> Result<()> {
        Ok(())
    }

    fn visit_term(&mut self, _term: &Term, codegen: &mut Codegen<'a>) -> Result<()> {
        Ok(())
    }

    fn visit_struct(&mut self, _stru: &Struct, codegen: &mut Codegen<'a>) -> Result<()> {
        Ok(())
    }

    fn write_bitcode(&self, name: &str) -> Result<bool> {
        //self.codegen.write_bitcode(name)?;
        Ok(true)
    }

    fn get_ir(&self) -> Result<Option<String>> {
        //Ok(Some(self.codegen.get_ir()))
        unimplemented!()
    }
}