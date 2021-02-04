use std::collections::HashSet;
use crate::ast::{IdTy, Error};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Default)]
pub(crate) struct SymbolTable {
    symbols: HashSet<IdTy>,
}

impl SymbolTable {
    pub fn lookup_symbol(&self, sym: &IdTy) -> bool {
        self.symbols.contains(sym)
    }

    pub fn insert(&mut self, sym: IdTy) -> Result<()> {
        if !self.lookup_symbol(&sym) {
            self.symbols.insert(sym);
            Ok(())
        }
        else {
            return Err(Error::SymbolAlreadyDefined(sym));
        }
    }
}