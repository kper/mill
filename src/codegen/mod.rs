use crate::symbol_table::*;
use llvm_sys::core::*;

use anyhow::Context;
use std::collections::HashMap;

use crate::ast::Identifier;
use crate::c_str;
use llvm_sys::prelude::*;

pub struct Codegen {
    pub context: LLVMContextRef,
    pub module: LLVMModuleRef,
    pub builder: LLVMBuilderRef,
    pub symbol_tables: HashMap<Identifier, LLVMSymbolTable>,
    pub function_table: LLVMFunctionTable,
    pub block_table: LLVMBlockTable,
    pub struct_table: LLVMStructTable,
    pub expr_tables: HashMap<Identifier, LLVMExprTable>,
}

impl Codegen {
    pub fn new(context: LLVMContextRef, module: LLVMModuleRef, builder: LLVMBuilderRef) -> Codegen {
        unsafe {
            LLVMSetTarget(module, c_str!("x86_64-unknown-linux-gnu"));

            Codegen {
                context,
                module,
                builder,
                symbol_tables: HashMap::default(),
                function_table: LLVMFunctionTable::default(),
                block_table: LLVMBlockTable::default(),
                struct_table: LLVMStructTable::default(),
                expr_tables: HashMap::default(),
            }
        }
    }

    pub fn clear_expr_table(&mut self, function: &Identifier) -> anyhow::Result<()> {
        self.expr_tables
            .get_mut(function)
            .with_context(|| "Cannot find expr table".to_string())?
            .clear()
    }
}
