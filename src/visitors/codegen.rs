use anyhow::Result;

use crate::Visitor;
use crate::visitors::CodegenVisitorTrait;
use crate::codegen::Codegen;
use crate::ast::*;
use log::{debug, warn};

use llvm_sys::bit_writer::*;
use llvm_sys::core::*;
use std::ffi::CString;
use std::ptr;
use crate::c_str;

use crate::symbol_table::*;

pub struct CodegenVisitor {
}

impl CodegenVisitor {
    pub fn new() -> Self {
        Self {
        }
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

            //let hello_world_str = LLVMBuildGlobalStringPtr(builder, c_str!("hello, world."), c_str!(""));

            //LLVMBuildRetVoid(builder);
        }

        //let context = codegen.get_context();

        //let builder = codegen.get_mut_builder();
        //let module = codegen.get_mut_module();
        //let context = codegen.get_context();
        //let ftable = codegen.get_mut_function_table();
        //let symbol_table = codegen.get_mut_symtable();

        /*
        let context = codegen.context;
        let i64_type = context.i64_type();
        let func_types = vec![i64_type.into(); func.pars.len()]; 
        let fn_type = i64_type.fn_type(&func_types, false);

        let function = codegen.get_mut_module().add_function(&func.id.get_name(), fn_type, None);

        codegen.get_mut_function_table().insert(&func.id.get_name(), function)?;

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

    fn visit_statement(&mut self, stmt: &Statement, codegen: &mut Codegen) -> Result<()> {
        debug!("{}: Visiting statement {:#?}", self.get_name(), stmt);

        //TODO add return void
        let context = codegen.context.clone();
        let module = codegen.module.clone();
        let builder = codegen.builder.clone();


        match stmt {
            Statement::Ret(_expr) => {
                //unimplemented!()

                unsafe {
                    // this is wrong
                    LLVMBuildRetVoid(builder);
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
    
    fn visit_guard(&mut self, _guard: &Guard, codegen: &mut Codegen) -> Result<()> {
        Ok(())
    }

    fn visit_expr(&mut self, expr: &Expr, codegen: &mut Codegen) -> Result<()> {
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

    fn write_bitcode(&self, name: &str) -> Result<bool> {
        //self.codegen.write_bitcode(name)?;
        Ok(true)
    }

    fn get_ir(&self) -> Result<Option<String>> {
        //Ok(Some(self.codegen.get_ir()))
        unimplemented!()
    }
}