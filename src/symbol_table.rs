use crate::ast::{Error, IdTy};
use inkwell::values::{BasicValue, BasicValueEnum};
use std::collections::HashMap;
use std::collections::HashSet;

type Result<T> = std::result::Result<T, Error>;
pub type Key = IdTy;

#[derive(Debug, Default)]
pub struct SymbolTable {
    symbols: HashSet<IdTy>,
}

impl SymbolTable {
    pub fn lookup_symbol(&self, sym: &IdTy) -> bool {
        self.symbols.contains(sym)
    }

    pub fn insert(&mut self, sym: &IdTy) -> Result<()> {
        if !self.lookup_symbol(sym) {
            self.symbols.insert(sym.clone());
            Ok(())
        } else {
            return Err(Error::SymbolAlreadyDefined(sym.to_string()));
        }
    }
}

#[derive(Debug, Hash, Clone)]
pub enum Value {
    Int(i64),
}

#[derive(Debug, Default)]
pub struct LLVMSymbolTable {
    symbols: HashMap<Key, Value>,
    counter: usize,
}

impl LLVMSymbolTable {
    pub fn lookup_symbol(&self, sym: &IdTy) -> bool {
        self.symbols.contains_key(sym)
    }

    pub fn get(&self, sym: &IdTy) -> Option<&Value> {
        self.symbols.get(sym)
    }

    pub fn insert(&mut self, sym: &IdTy, val: Value) -> Result<()> {
        if !self.lookup_symbol(sym) {
            self.symbols.insert(sym.clone(), val);
            Ok(())
        } else {
            return Err(Error::SymbolAlreadyDefined(sym.to_string()));
        }
    }
}
