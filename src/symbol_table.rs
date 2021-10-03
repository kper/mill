#![allow(dead_code)]

use crate::ast::{Identifier, Struct};
use anyhow::{bail, Result};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use llvm_sys::core::*;
use llvm_sys::prelude::*;
use crate::c_str;

pub type Key = String;

#[derive(Debug, Clone, PartialEq)]
pub enum BasicValueType {
    Int(LLVMTypeRef),
    Pointer,
}

#[derive(Debug, Clone)]
pub struct BasicValue {
    pub ty: BasicValueType,
    pub value: LLVMValueRef
}

impl BasicValue {
    pub fn load(&self, _context: LLVMContextRef, builder: LLVMBuilderRef, id: &Identifier) -> Result<LLVMValueRef> {
        if matches!(self.ty, BasicValueType::Pointer) {
            unsafe {
                let ptr = LLVMBuildLoad(builder, self.value, c_str!(id.get_name()));

                return Ok(ptr);
            }
        }
        else {
            bail!("Loaded value has to be a pointer");
        }
    }

    pub fn store(&self, context: LLVMContextRef, builder: LLVMBuilderRef, id: &Identifier) -> Result<LLVMValueRef> {
        let ptr = self.ty.alloca(context, builder, id)?;

        unsafe {
            let res = LLVMBuildStore(builder, self.value, ptr.value);

            return Ok(res);
        }
    }
}

impl BasicValueType {
    pub fn alloca(&self, context: LLVMContextRef, builder: LLVMBuilderRef, id: &Identifier) -> Result<BasicValue> {
        unsafe {
            let ty = match &self {
                BasicValueType::Int(_) => LLVMIntTypeInContext(context, 64),
                BasicValueType::Pointer => bail!("Cannot alloca a pointer"),
            };

            let value_ref = LLVMBuildAlloca(builder, ty, c_str!(id));
            let value = BasicValue {
                ty: BasicValueType::Pointer,
                value: value_ref
            };

            Ok(value)
        }
    }

    pub fn get_ty(&self) -> LLVMTypeRef {
        match &self {
            BasicValueType::Int(x) => x.clone(),
            BasicValueType::Pointer => panic!("Cannot get ty of a pointer"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionType(LLVMTypeRef);

#[derive(Debug, Default, Clone)]
pub struct SymbolTable {
    symbols: HashSet<Key>,
}

impl SymbolTable {
    pub fn lookup_symbol(&self, sym: &Key) -> bool {
        self.symbols.contains(sym)
    }

    pub fn insert(&mut self, sym: &Key) -> Result<()> {
        if !self.lookup_symbol(sym) {
            self.symbols.insert(sym.clone());
            Ok(())
        } else {
            bail!("Symbol {} is already defined", sym);
        }
    }
}

/**
 * Keeps the information for a statement in a stack.
 */
#[derive(Debug, Default, Clone)]
pub struct LLVMExprTable {
    stack: VecDeque<BasicValue>,
}

impl LLVMExprTable {
    /**
     * Get the last LLVMValue from the stack.
     */
    pub fn get_last(&mut self) -> Option<BasicValue> {
        self.stack.pop_front()
    }

    /**
     * Clears all values from the stack.
     */
    pub fn clear(&mut self) -> Result<()> {
        self.stack.clear();

        Ok(())
    }

    /**
     * Add a value to the back.
     */
    pub fn push(&mut self, value: LLVMValueRef, ty: BasicValueType) -> Result<()> {
        self.stack.push_back(BasicValue {
            value,
            ty
        });

        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub struct LLVMSymbolTable {
    symbols: HashMap<Key, (Identifier, BasicValue)>,
    counter: usize,
}



impl LLVMSymbolTable {
    pub fn lookup_symbol(&self, sym: &Key) -> bool {
        self.symbols.contains_key(sym)
    }

    pub fn get(&self, sym: &Key) -> Option<&BasicValue> {
        self.symbols.get(sym).as_ref().map(|x| &x.1)
    }

    pub fn get_identifier(&self, sym: &Key) -> Option<&Identifier> {
        self.symbols.get(sym).as_ref().map(|x| &x.0)
    }

    pub fn get_both(&self, sym: &Key) -> Option<&(Identifier, BasicValue)> {
        self.symbols.get(sym)
    }

    pub fn insert(&mut self, sym: &Key, val: (Identifier, BasicValue)) -> Result<()> {
        if !self.lookup_symbol(sym) {
            self.symbols.insert(sym.clone(), (val.0, val.1));
            Ok(())
        } else {
            bail!("Symbol {} is already defined", sym);
        }
    }

    pub fn clear(&mut self) {
        self.symbols.clear();
    }

    pub fn get_last_sym(&self) -> Option<(Identifier, BasicValue)> {
        let curr = format!("{}", self.counter);
        let x = self.symbols.get(&curr).clone();

        if let Some((id, value)) = x {
            Some((id.clone(), value.clone()))
        }
        else {
            None
        }
    }

    pub fn get_new_name(&mut self) -> String {
        let val = self.counter;
        let sval = format!("{}", val);
        self.counter += 1;

        sval
    }
}

#[derive(Debug, Default)]
pub struct LLVMFunctionTable {
    symbols: HashMap<Key, LLVMValueRef>,
    counter: usize,
}

impl LLVMFunctionTable {
    pub fn lookup_symbol(&self, sym: &Key) -> bool {
        self.symbols.contains_key(sym)
    }

    pub fn get(&self, sym: &Key) -> Option<&LLVMValueRef> {
        self.symbols.get(sym)
    }

    pub fn insert(&mut self, sym: &Key, val: LLVMValueRef) -> Result<()> {
        if !self.lookup_symbol(sym) {
            self.symbols.insert(sym.clone(), val);
            Ok(())
        } else {
            bail!("Symbol {} is already defined", sym);
        }
    }

    pub fn clear(&mut self) {
        self.symbols.clear();
    }

    pub fn get_new_name(&mut self) -> String {
        let val = self.counter;
        let sval = format!("{}", val);
        self.counter += 1;

        sval
    }
}

#[derive(Debug, Default)]
pub struct LLVMBlockTable {
    symbols: HashMap<Key, (LLVMBasicBlockRef)>,
    counter: usize,
}

impl<'a> LLVMBlockTable {
    pub fn lookup_symbol(&self, sym: &Key) -> bool {
        self.symbols.contains_key(sym)
    }

    pub fn get(&self, sym: &Key) -> Option<&(LLVMBasicBlockRef)> {
        self.symbols.get(sym)
    }

    pub fn insert(&mut self, sym: &Key, val: (LLVMBasicBlockRef)) -> Result<()> {
        if !self.lookup_symbol(sym) {
            self.symbols.insert(sym.clone(), val);
            Ok(())
        } else {
            bail!("Symbol {} is already defined", sym);
        }
    }

    pub fn clear(&mut self) {
        self.symbols.clear();
    }

    pub fn get_new_name(&mut self) -> String {
        let val = self.counter;
        let sval = format!("{}", val);
        self.counter += 1;

        sval
    }
}

#[derive(Debug, Default)]
pub struct LLVMStructTable {
    symbols: HashMap<Key, (Struct, LLVMTypeRef)>,
    counter: usize,
}

impl LLVMStructTable {
    pub fn lookup_symbol(&self, sym: &Key) -> bool {
        self.symbols.contains_key(sym)
    }

    pub fn get(&self, sym: &Key) -> Option<&(Struct, LLVMTypeRef)> {
        self.symbols.get(sym)
    }

    pub fn insert(&mut self, sym: &Key, val: (Struct, LLVMTypeRef)) -> Result<()> {
        if !self.lookup_symbol(sym) {
            self.symbols.insert(sym.clone(), val);
            Ok(())
        } else {
            bail!("Symbol {} is already defined", sym);
        }
    }

    pub fn clear(&mut self) {
        self.symbols.clear();
    }

    pub fn get_new_name(&mut self) -> String {
        let val = self.counter;
        let sval = format!("{}", val);
        self.counter += 1;

        sval
    }
}
