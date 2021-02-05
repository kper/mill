use crate::ast::*;
use crate::symbol_table::SymbolTable;
use crate::visitors::CodegenVisitor;
use std::path::Path;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::targets::{InitializationConfig, Target};
use inkwell::values::FunctionValue;
use inkwell::OptimizationLevel;

type Result<T> = std::result::Result<T, Error>;

pub struct Codegen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> Codegen<'ctx> {
    pub fn new(context: &'ctx Context, module: Module<'ctx>) -> Codegen<'ctx> {
        /*
        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();*/

        Target::initialize_native(&InitializationConfig::default())
            .expect("Failed to initialize native target");

        let execution_engine = module.create_execution_engine().unwrap();

        Codegen {
            context: context,
            module,
            builder: context.create_builder(),
            execution_engine,
        }
    }

    pub fn write_bitcode(&self, name: &str) -> Result<()> {
        let path = Path::new(name);

        self.module.write_bitcode_to_path(path);

        Ok(())
    }
}

impl<'ctx> CodegenVisitor<'ctx> for Program {
    fn visit(&self, codegen: &'ctx mut Codegen, functions: &SymbolTable) -> Result<()> {
        for func in &self.functions {
            <Func as CodegenVisitor>::visit(&func, codegen, functions)?;
        }

        Ok(())
    }
}

impl<'ctx> CodegenVisitor<'ctx> for Func {
    fn visit(&self, codegen: &'ctx mut Codegen, functions: &SymbolTable) -> Result<()> {
        let context = &codegen.context;
        let module = &codegen.module;
        let builder = &codegen.builder;

        let i64_type = context.i64_type();
        let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into()], false);
        let function = module.add_function(&self.id, fn_type, None);
        let basic_block = context.append_basic_block(function, &self.id);

        builder.position_at_end(basic_block);

        for stmt in &self.statements {
            stmt.codegen(codegen, &function, functions)?;
        }

        Ok(())
    }
}

impl Statement {
    pub fn codegen<'ctx>(
        &self,
        codegen: &'ctx mut Codegen,
        function: &'ctx FunctionValue,
        functions: &SymbolTable,
    ) -> Result<()> {
        let x = function.get_nth_param(0).unwrap().into_int_value();
        let y = function.get_nth_param(1).unwrap().into_int_value();

        let sum = codegen.builder.build_int_add(x, y, "sum");

        codegen.builder.build_return(Some(&sum));

        Ok(())
    }
}
