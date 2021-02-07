#![allow(dead_code)]

use anyhow::{bail, Result};
use inkwell::basic_block::BasicBlock;
use inkwell::types::StructType;
use inkwell::values::{BasicValueEnum, FunctionValue};
use std::collections::HashMap;
use std::collections::HashSet;

pub type Key = String;

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
pub struct LLVMSymbolTable<'a> {
    symbols: HashMap<Key, BasicValueEnum<'a>>,
    counter: usize,
}

impl<'a> LLVMSymbolTable<'a> {
    pub fn lookup_symbol(&self, sym: &Key) -> bool {
        self.symbols.contains_key(sym)
    }

    pub fn get(&self, sym: &Key) -> Option<&BasicValueEnum<'a>> {
        self.symbols.get(sym)
    }

    pub fn insert(&mut self, sym: &Key, val: BasicValueEnum<'a>) -> Result<()> {
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
    pub fn lookup_symbol(&self, sym: &Key) -> bool {
        self.symbols.contains_key(sym)
    }

    pub fn get(&self, sym: &Key) -> Option<&FunctionValue<'a>> {
        self.symbols.get(sym)
    }

    pub fn insert(&mut self, sym: &Key, val: FunctionValue<'a>) -> Result<()> {
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
    pub fn lookup_symbol(&self, sym: &Key) -> bool {
        self.symbols.contains_key(sym)
    }

    pub fn get(&self, sym: &Key) -> Option<&(BasicBlock<'a>, BasicBlock<'a>)> {
        self.symbols.get(sym)
    }

    pub fn insert(&mut self, sym: &Key, val: (BasicBlock<'a>, BasicBlock<'a>)) -> Result<()> {
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
pub struct LLVMStructTable<'a> {
    symbols: HashMap<Key, StructType<'a>>,
    counter: usize,
}

impl<'a> LLVMStructTable<'a> {
    pub fn lookup_symbol(&self, sym: &Key) -> bool {
        self.symbols.contains_key(sym)
    }

    pub fn get(&self, sym: &Key) -> Option<&StructType<'a>> {
        self.symbols.get(sym)
    }

    pub fn insert(&mut self, sym: &Key, val: StructType<'a>) -> Result<()> {
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
