use crate::ast::*;
use crate::symbol_table::{Key, LLVMSymbolTable, SymbolTable, Value};
use crate::visitors::CodegenVisitor;
use std::borrow::Cow;
use std::path::Path;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::targets::{InitializationConfig, Target};
use inkwell::values::{BasicValue, BasicValueEnum, FunctionValue};
use inkwell::OptimizationLevel;

type Result<T> = std::result::Result<T, Error>;

pub struct Codegen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
    symbol_table: LLVMSymbolTable<'ctx>,
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
            symbol_table: LLVMSymbolTable::default(),
        }
    }

    pub fn write_bitcode(&self, name: &str) -> Result<()> {
        let path = Path::new(name);

        self.module.write_bitcode_to_path(path);

        Ok(())
    }
}

impl<'ctx> CodegenVisitor<'ctx> for Codegen<'ctx> {
    fn visit_program(&mut self, program: &mut Program) -> Result<()> {
        for func in program.functions.iter_mut() {
            //<Func as CodegenVisitor>::visit(&func, codegen, functions)?;
            self.visit_func(func)?;
        }

        Ok(())
    }

    fn visit_func(&mut self, func: &mut Func) -> Result<()> {
        let context = &self.context;
        let module = &self.module;
        let builder = &self.builder;

        let i64_type = context.i64_type();

        let func_types = vec![i64_type.into(); func.pars.len()];
        let fn_type = i64_type.fn_type(&func_types, false);
        let function = module.add_function(&func.id, fn_type, None);
        let basic_block = context.append_basic_block(function, &func.id);

        builder.position_at_end(basic_block);

        //let x = llvm_function.get_nth_param(0).unwrap().into_int_value();
        //let y = llvm_function.get_nth_param(1).unwrap().into_int_value();
        for stmt in func.statements.iter() {
            self.visit_statement(stmt)?;
        }

        self.symbol_table.clear();

        Ok(())
    }

    fn visit_statement(&mut self, stmt: &Statement) -> Result<()> {
        //let sum = self.builder.build_int_add(x, y, "sum");
        //self.builder.build_return(Some(&sum));

        match stmt {
            Statement::Ret(expr) => {
                let res = self.visit_expr(expr).map(|x| x.into_owned());
                let ret: Option<&dyn BasicValue> = res.as_ref().map(|x| x as &dyn BasicValue);

                self.builder.build_return(ret);
            }
            _ => unimplemented!(),
        }

        Ok(())
    }

    fn visit_expr(&mut self, expr: &Expr) -> Option<Cow<BasicValueEnum<'ctx>>> {
        match expr {
            Expr::Single(term) => {
                return self.visit_term(term);
            }
            _ => return None,
        }
    }

    fn visit_term(&mut self, term: &Term) -> Option<Cow<BasicValueEnum<'ctx>>> {
        match term {
            Term::Num(num) => {
                let i64_type = self.context.i64_type();
                let obj = BasicValueEnum::IntValue(i64_type.const_int(*num as u64, false));

                return Some(Cow::Owned(obj));
            }
            Term::Id(id) => {
                return self.symbol_table.get(id).map(|x| Cow::Borrowed(x));
            }
            _ => return None,
        }
    }
}

/*
impl<'ctx> CodegenVisitor<'ctx> for Program {
    fn visit_program(&self, functions: &SymbolTable) -> Result<()>;
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
*/

/*
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
*/
