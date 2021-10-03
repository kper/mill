#![allow(dead_code)]

use crate::ast::{Identifier, Struct};
use anyhow::{bail, Result};
use std::collections::HashMap;
use std::collections::HashSet;

use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::prelude::LLVMBasicBlockRef;

pub type Key = String;

#[derive(Debug, Clone)]
pub enum BasicValueType {
    Int(LLVMTypeRef)
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

#[derive(Debug, Default, Clone)]
pub struct LLVMSymbolTable {
    symbols: HashMap<Key, (Identifier, BasicValueType)>,
    counter: usize,
}

impl LLVMSymbolTable {
    pub fn lookup_symbol(&self, sym: &Key) -> bool {
        self.symbols.contains_key(sym)
    }

    pub fn get(&self, sym: &Key) -> Option<&BasicValueType> {
        self.symbols.get(sym).as_ref().map(|x| &x.1)
    }

    pub fn get_identifier(&self, sym: &Key) -> Option<&Identifier> {
        self.symbols.get(sym).as_ref().map(|x| &x.0)
    }

    pub fn get_both(&self, sym: &Key) -> Option<&(Identifier, BasicValueType)> {
        self.symbols.get(sym)
    }

    pub fn insert(&mut self, sym: &Key, val: (Identifier, LLVMTypeRef)) -> Result<()> {
        if !self.lookup_symbol(sym) {
            self.symbols.insert(sym.clone(), (val.0, BasicValueType::Int(val.1)));
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
pub struct LLVMFunctionTable {
    symbols: HashMap<Key, FunctionType>,
    counter: usize,
}

impl LLVMFunctionTable {
    pub fn lookup_symbol(&self, sym: &Key) -> bool {
        self.symbols.contains_key(sym)
    }

    pub fn get(&self, sym: &Key) -> Option<&FunctionType> {
        self.symbols.get(sym)
    }

    pub fn insert(&mut self, sym: &Key, val: FunctionType) -> Result<()> {
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
    symbols: HashMap<Key, (LLVMBasicBlockRef, LLVMBasicBlockRef)>,
    counter: usize,
}

impl<'a> LLVMBlockTable {
    pub fn lookup_symbol(&self, sym: &Key) -> bool {
        self.symbols.contains_key(sym)
    }

    pub fn get(&self, sym: &Key) -> Option<&(LLVMBasicBlockRef, LLVMBasicBlockRef)> {
        self.symbols.get(sym)
    }

    pub fn insert(&mut self, sym: &Key, val: (LLVMBasicBlockRef, LLVMBasicBlockRef)) -> Result<()> {
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
