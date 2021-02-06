use crate::ast::{Error, IdTy};
use inkwell::values::{BasicValueEnum};
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

#[derive(Debug, Default)]
pub struct LLVMSymbolTable<'a> {
    symbols: HashMap<Key, BasicValueEnum<'a>>,
    counter: usize,
}

impl<'a> LLVMSymbolTable<'a> {
    pub fn lookup_symbol(&self, sym: &IdTy) -> bool {
        self.symbols.contains_key(sym)
    }

    pub fn get(&self, sym: &IdTy) -> Option<&BasicValueEnum<'a>> {
        self.symbols.get(sym)
    }

    pub fn insert(&mut self, sym: &IdTy, val: BasicValueEnum<'a>) -> Result<()> {
        if !self.lookup_symbol(sym) {
            self.symbols.insert(sym.clone(), val);
            Ok(())
        } else {
            return Err(Error::SymbolAlreadyDefined(sym.to_string()));
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
