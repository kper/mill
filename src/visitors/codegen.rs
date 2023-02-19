use std::collections::VecDeque;
use anyhow::{Context, Result};

use crate::visitors::CodegenVisitorTrait;
use crate::codegen::{Codegen};
use crate::ast::*;
use log::{debug, warn};

use llvm_sys::core::*;
use llvm_sys::prelude::LLVMTypeRef;
use crate::c_str;

use crate::symbol_table::*;

pub struct CodegenVisitor {
    number_generator: usize,
}

impl CodegenVisitor {
    pub fn new() -> Self {
        Self {
            number_generator: 0,
        }
    }

    fn generate_number(&mut self) -> usize {
        let number = self.number_generator;
        self.number_generator += 1;
        number
    }

    unsafe fn get_void_ty(&self, codegen: &mut Codegen) -> LLVMTypeRef {
        LLVMVoidTypeInContext(codegen.context)
    }

    unsafe fn get_i8_ty(&self, codegen: &mut Codegen) -> LLVMTypeRef {
        LLVMIntTypeInContext(codegen.context, 8)
    }

    unsafe fn get_i32_ty(&self, codegen: &mut Codegen) -> LLVMTypeRef {
        LLVMIntTypeInContext(codegen.context, 32)
    }

    unsafe fn get_basic_value_ty_from_datatype(&self, ty: &DataType, codegen: &mut Codegen) -> BasicValueType {
        match ty {
            DataType::Int => BasicValueType::Int(self.get_i32_ty(codegen)),
            _ => unimplemented!()
        }
    }

    unsafe fn get_llvm_function_ty_by_function_signature(&mut self,
                                                         function_signature: &FunctionSignature,
                                                         codegen: &mut Codegen) -> LLVMTypeRef {
        let mut args_ty : Vec<_> = function_signature.get_args_ty().iter().map(|arg| {
            match arg {
                DataType::Int => self.get_i32_ty(codegen),
                _ => unimplemented!()
            }
        }).collect();

        let ret_ty = match function_signature.get_ret_ty() {
            Some(DataType::Int) => self.get_i32_ty(codegen),
            None => self.get_void_ty(codegen),
            _ => unimplemented!()
        };

        LLVMFunctionType(ret_ty, args_ty.as_mut_ptr(), args_ty.len() as u32, 0)
    }
}

impl CodegenVisitorTrait for CodegenVisitor {
    fn get_name(&self) -> String {
        "CodegenVisitor".to_string()
    }

    fn visit_program(&mut self, _program: &Program, _codegen: &mut Codegen) -> Result<()> {
        debug!("{}: running visit_program", self.get_name());
        Ok(())
    }

    fn set_block_position_to_function(&mut self, func: &Func, codegen: &mut Codegen) -> Result<()> {
        let builder = codegen.builder;
        let block = codegen.block_table
            .get(func.id.get_name())
            .with_context(|| "Cannot find block of the function".to_string())?;

        unsafe {
            LLVMPositionBuilderAtEnd(builder, *block);
        }

        Ok(())
    }

    fn visit_func(&mut self, func: &Func, codegen: &mut Codegen) -> Result<()> {
        debug!("{}: running visit_func", self.get_name());

        let context = codegen.context.clone();
        let module = codegen.module.clone();

        unsafe {
            let func_type = self.get_llvm_function_ty_by_function_signature(&func.get_signature(), codegen);
            let func_llvm = LLVMAddFunction(module, c_str!(func.id.get_name()), func_type);
            let block = LLVMAppendBasicBlockInContext(context, func_llvm, c_str!(func.id.get_name()));

            codegen.block_table.insert(func.id.get_name(), block)?;
            codegen.function_table.insert(&func.id.get_name(), func.get_signature(), func_llvm)?;
            codegen.expr_tables.insert(func.id.clone(), LLVMExprTable::default());
            codegen.symbol_tables.insert(func.id.clone(), LLVMSymbolTable::default());
            debug!("Added function '{}' to function table", func.id.get_name());
        }

        Ok(())
    }

    fn visit_param(&mut self, func: &Func, param: &IdTy, codegen: &mut Codegen) -> Result<()> {
        let (_, llvm_func) = codegen.function_table.get(&func.id.get_name()).with_context(|| "Cannot find function".to_string())?;
        codegen.symbol_tables.get_mut(&func.id)
            .with_context(|| "Cannot find symbol table of function".to_string())?
            .insert(param.get_name(), (param.clone(), BasicValue {
                ty: BasicValueType::Function,
                value: llvm_func.clone()
            }))
            .with_context(|| "Cannot insert into symbol".to_string())?;

        Ok(())
    }

    fn visit_statement(&mut self, func: &Func, stmt: &Statement, codegen: &mut Codegen) -> Result<()> {
        debug!("{}: Visiting statement {:#?}", self.get_name(), stmt);

        let context = codegen.context.clone();
        let _module = codegen.module.clone();
        let builder = codegen.builder.clone();

        match stmt {
            Statement::RetVoid => {
                unsafe {
                    LLVMBuildRetVoid(builder);
                }
            }
            Statement::Ret(expr) => {
                self.visit_expr(func, stmt, expr, codegen)?;

                let value = codegen.expr_tables
                    .get_mut(&func.id)
                    .with_context(|| "Cannot get expr table")?
                    .get_last()
                    .with_context(|| format!("Failed to fetch last expression value when returning from function '{}'", func.id.get_name()))?;

                unsafe {
                    LLVMBuildRet(builder, value.value);
                }
            }
            Statement::Assign(id, expr) => {
                self.visit_expr(func, stmt, expr, codegen)?;

                let sym_expr = codegen.expr_tables
                    .get_mut(&func.id)
                    .with_context(|| "Cannot get expr table")?
                    .get_last();

                if let Some(value) = sym_expr {
                    debug!("Building bit cast for {}", id.get_name());

                    unsafe {
                        let i64_ty = LLVMIntTypeInContext(codegen.context, 64);
                        let ty = BasicValueType::Int(i64_ty);

                        let _ = value.store(context, builder, id)?;

                        codegen.symbol_tables
                            .get_mut(&func.id)
                            .with_context(|| "Cannot get symbol table")?
                            .insert(id.get_name(), (id.clone(), BasicValue {
                            ty: ty.clone(),
                            value: value.value,
                        }))?;
                    }
                }
                else {
                    warn!("No last sym for the assignment statement");
                }
            }
            _ => {

            }
        }

        codegen.clear_expr_table(&func.id)?;

        Ok(())
    }

    fn visit_expr(&mut self, func: &Func, statement: &Statement, expr: &Expr, codegen: &mut Codegen) -> Result<()> {
        let builder = codegen.builder.clone();
        debug!("{}: Visiting expr {:#?}", self.get_name(), expr);

        match expr {
            Expr::Single(term) => {
                self.visit_term(func, statement, expr, term, codegen)?;
            }
            Expr::Num(num) => {
                unsafe {
                    let i64_ty = LLVMIntTypeInContext(codegen.context, 64);
                    let value = LLVMConstInt(i64_ty, *num as u64, 0);
                    codegen.expr_tables
                        .get_mut(&func.id)
                        .with_context(|| "Cannot get expr table")?
                        .push(value, BasicValueType::Int(i64_ty))?;
                }
            }
            Expr::Dual(opcode, term1, term2) => {
                self.visit_term(func, statement, expr, term1, codegen)?;
                self.visit_term(func, statement, expr, term2, codegen)?;

                let t2 = codegen.expr_tables
                    .get_mut(&func.id)
                    .with_context(|| "Cannot get expr table")?
                    .get_last().with_context(|| "Cannot get the second term of the operation")?;
                let t1 = codegen.expr_tables
                    .get_mut(&func.id)
                    .with_context(|| "Cannot get expr table")?
                    .get_last().with_context(|| "Cannot get the first term of the operation")?;

                unsafe {
                    match opcode {
                        Opcode::Add =>{
                            let name = c_str!(&self.generate_number().to_string());
                            let value = LLVMBuildAdd(builder, t1.value, t2.value, name);

                            let i64_ty = LLVMIntTypeInContext(codegen.context, 64);
                            codegen.expr_tables
                                .get_mut(&func.id)
                                .with_context(|| "Cannot get expr table")?
                                .push(value, BasicValueType::Int(i64_ty))?;
                        }
                        _ => unimplemented!()
                    }
                }
            }
            Expr::Call(ident, exprs) => {
                for argument in exprs {
                    self.visit_expr(func, statement, argument, codegen)
                        .with_context(|| "Failed when visiting expressions of the arguments of a function call".to_string())?;
                }

                debug!("Building function call for '{}'", ident);

                let len = exprs.len();
                let mut args = VecDeque::with_capacity(len);

                for _ in 0..len {
                    args.push_front(codegen.expr_tables
                        .get_mut(&func.id)
                        .with_context(|| "Cannot get expr table")?
                        .get_last()
                        .with_context(|| "Cannot get last expression for arguments of a function call".to_string())?.value);
                }

                debug!("Removed '{}' arguments from expr table", len);

                unsafe {
                    let (function_signature, function_ref) = codegen.function_table.get_mut(ident.get_name()).with_context(|| format!("Cannot find function '{}'", ident)) .map(|(function_signature, function_ref)| (function_signature.clone(), *function_ref))?; if let Some(ret_ty) = function_signature.get_ret_ty() {
                        let basic_ty = self.get_basic_value_ty_from_datatype(ret_ty, codegen);
                        let value = LLVMBuildCall(builder, function_ref , args.make_contiguous().as_mut_ptr(), 0, c_str!(self.generate_number()));
                        codegen.expr_tables
                            .get_mut(&func.id)
                            .with_context(|| "Cannot find function".to_string())?
                            .push(value, basic_ty)
                            .with_context(|| "Cannot add expression to the expr table")?;
                        debug!("Added expr to expr table");
                    }
                }
            }
            _ => {
                unimplemented!()
            }
        }

        Ok(())
    }

    fn visit_term(&mut self, func: &Func, _statement: &Statement, _expr: &Expr, term: &Term, codegen: &mut Codegen) -> Result<()> {
        debug!("{}: Visiting term {:#?}", self.get_name(), term);

        match term {
            Term::Num(num) => {
                unsafe {
                    let i64_ty = LLVMIntTypeInContext(codegen.context, 64);
                    let value = LLVMConstInt(i64_ty, *num as u64, 0);
                    codegen.expr_tables
                        .get_mut(&func.id)
                        .with_context(|| "Cannot get expr table")?
                        .push(value, BasicValueType::Int(i64_ty))?;
                }
            }
            Term::Id(ident) => {
                let sym = codegen.symbol_tables
                    .get(&func.id)
                    .with_context(|| "Cannot get expr table")?
                    .get(ident.get_name()).with_context(|| format!("Cannot find identifier '{:?}' in symbol table", ident))?;
                codegen.expr_tables
                    .get_mut(&func.id)
                    .with_context(|| "Cannot get expr table")?
                    .push(sym.value, sym.ty.clone()).with_context(|| format!("Cannot push symbol '{:?}' to expr table", ident))?;
                debug!("Added term '{:?}' to expr table", ident);
            }
            _ => {
                unimplemented!();
            }
        }

        Ok(())
    }

    fn visit_struct(&mut self, _stru: &Struct, _codegen: &mut Codegen) -> Result<()> {
        Ok(())
    }
}