use crate::ast::*;
use crate::symbol_table::*;
use crate::visitors::CodegenVisitor;
use std::borrow::Cow;
use std::path::Path;

use llvm_sys::core::*;

use anyhow::{bail, Context, Result};
use log::debug;

use llvm_sys::core::*;
use llvm_sys::prelude::*;
use crate::c_str;

pub struct Codegen {
    pub context: LLVMContextRef,
    pub module: LLVMModuleRef,
    pub builder: LLVMBuilderRef,
    //execution_engine: ExecutionEngine<'ctx>,
    pub symbol_table: LLVMSymbolTable,
    pub function_table: LLVMFunctionTable,
    pub block_table: LLVMBlockTable,
    pub struct_table: LLVMStructTable,
    pub expr_table: LLVMExprTable,
}

impl Codegen {
    pub fn new(context: LLVMContextRef, module: LLVMModuleRef, builder: LLVMBuilderRef) -> Codegen {
        unsafe {
            LLVMSetTarget(module, c_str!("x86_64-unknown-linux-gnu"));

            Codegen {
                context,
                module,
                builder,
                //execution_engine,
                symbol_table: LLVMSymbolTable::default(),
                function_table: LLVMFunctionTable::default(),
                block_table: LLVMBlockTable::default(),
                struct_table: LLVMStructTable::default(),
                expr_table: LLVMExprTable::default(),
            }
        }
    }
}
