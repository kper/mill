#![allow(dead_code)]

use crate::ast::IdTy;
use anyhow::{bail, Result};
use inkwell::basic_block::BasicBlock;
use inkwell::values::{BasicValueEnum, FunctionValue};
use std::collections::HashMap;
use std::collections::HashSet;

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
            bail!("Symbol {} is already defined", sym);
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
pub struct LLVMFunctionTable<'a> {
    symbols: HashMap<Key, FunctionValue<'a>>,
    counter: usize,
}

impl<'a> LLVMFunctionTable<'a> {
    pub fn lookup_symbol(&self, sym: &IdTy) -> bool {
        self.symbols.contains_key(sym)
    }

    pub fn get(&self, sym: &IdTy) -> Option<&FunctionValue<'a>> {
        self.symbols.get(sym)
    }

    pub fn insert(&mut self, sym: &IdTy, val: FunctionValue<'a>) -> Result<()> {
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
pub struct LLVMBlockTable<'a> {
    symbols: HashMap<Key, (BasicBlock<'a>, BasicBlock<'a>)>,
    counter: usize,
}

impl<'a> LLVMBlockTable<'a> {
    pub fn lookup_symbol(&self, sym: &IdTy) -> bool {
        self.symbols.contains_key(sym)
    }

    pub fn get(&self, sym: &IdTy) -> Option<&(BasicBlock<'a>, BasicBlock<'a>)> {
        self.symbols.get(sym)
    }

    pub fn insert(&mut self, sym: &IdTy, val: (BasicBlock<'a>, BasicBlock<'a>)) -> Result<()> {
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
