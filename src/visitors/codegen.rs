use anyhow::{Context, Result};

use crate::Visitor;
use crate::visitors::CodegenVisitorTrait;
use crate::codegen::{Codegen, MRef};
use crate::ast::*;
use log::{debug, warn};

use llvm_sys::bit_writer::*;
use llvm_sys::core::*;
use std::ffi::CString;
use std::ptr;
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
}

impl CodegenVisitorTrait for CodegenVisitor {
    fn get_name(&self) -> String {
        "CodegenVisitor".to_string()
    }

    fn visit_program(&mut self, _program: &Program, codegen: &mut Codegen) -> Result<()> {
        debug!("{}: running visit_program", self.get_name());
        Ok(())
    }

    fn visit_func(&mut self, func: &Func, codegen: &mut Codegen) -> Result<()> {
        debug!("{}: running visit_func", self.get_name());

        let context = codegen.context.clone();
        let module = codegen.module.clone();
        let builder = codegen.builder.clone();

        unsafe {
            let void_type = LLVMVoidTypeInContext(context);
            let i8_type = LLVMIntTypeInContext(context, 8);
            let _i8_pointer_type = LLVMPointerType(i8_type, 0);

            let func_type = LLVMFunctionType(void_type, ptr::null_mut(), 0, 0);
            let func_llvm = LLVMAddFunction(module, c_str!(func.id.get_name()), func_type);
            let block = LLVMAppendBasicBlockInContext(context, func_llvm, c_str!(func.id.get_name()));
            LLVMPositionBuilderAtEnd(builder, block);

            codegen.block_table.insert(func.id.get_name(), block)?;
            codegen.function_table.insert(&func.id.get_name(), func_llvm)?;

        }

        Ok(())
    }

    fn visit_statement(&mut self, stmt: &Statement, codegen: &mut Codegen) -> Result<()> {
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
            Statement::Ret(_expr) => {
                // The traversing order will evaluate the expression before the statement.
                let value = codegen.expr_table.get_last().with_context(|| "Failed to fetch last expression value".to_string())?;

                unsafe {
                    LLVMBuildRet(builder, value.value);
                }
            }
            Statement::Assign(id, _expr) => {
                let sym_expr = codegen.expr_table.get_last();

                if let Some(value) = sym_expr {
                    debug!("Building bit cast for {}", id.get_name());

                    unsafe {
                        let i64_ty = LLVMIntTypeInContext(codegen.context, 64);
                        let ty = BasicValueType::Int(i64_ty);

                        let _ = value.store(context, builder, id)?;

                        codegen.symbol_table.insert(id.get_name(), (id.clone(), BasicValue {
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

        Ok(())
    }

    fn visit_expr(&mut self, expr: &Expr, codegen: &mut Codegen) -> Result<()> {
        let builder = codegen.builder.clone();
        debug!("{}: Visiting expr {:#?}", self.get_name(), expr);

        match expr {
            Expr::Single(_term) => {
                // do nothing
            }
            Expr::Num(num) => {
                unsafe {
                    let i64_ty = LLVMIntTypeInContext(codegen.context, 64);
                    let value = LLVMConstInt(i64_ty, *num as u64, 0);
                    codegen.expr_table.push(value, BasicValueType::Int(i64_ty))?;
                }
            }
            Expr::Dual(opcode, term1, term2) => {
                let t2 = codegen.expr_table.get_last().with_context(|| "Cannot get the second term of the operation")?;
                let t1 = codegen.expr_table.get_last().with_context(|| "Cannot get the first term of the operation")?;

                unsafe {
                    match opcode {
                        Opcode::Add =>{
                            let name = c_str!(&self.generate_number().to_string());
                            let value = LLVMBuildAdd(builder, t1.value, t2.value, name);

                            let i64_ty = LLVMIntTypeInContext(codegen.context, 64);
                            codegen.expr_table.push(value, BasicValueType::Int(i64_ty))?;
                        }
                        _ => unimplemented!()
                    }
                }
            }
            _ => {
                unimplemented!()
            }
        }

        Ok(())
    }

    fn visit_term(&mut self, term: &Term, codegen: &mut Codegen) -> Result<()> {
        debug!("{}: Visiting term {:#?}", self.get_name(), term);

        match term {
            Term::Num(num) => {
                unsafe {
                    let i64_ty = LLVMIntTypeInContext(codegen.context, 64);
                    let value = LLVMConstInt(i64_ty, *num as u64, 0);
                    codegen.expr_table.push(value, BasicValueType::Int(i64_ty))?;
                }
            }
            _ => {
                unimplemented!();
            }
        }

        Ok(())
    }

    fn visit_struct(&mut self, _stru: &Struct, codegen: &mut Codegen) -> Result<()> {
        Ok(())
    }
}