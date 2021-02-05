use crate::symbol_table::SymbolTable;
use crate::ast::IdTy;
use crate::ast::Error;

type Result<T> = std::result::Result<T, Error>;

pub trait CheckIfFunctionCallExistsVisitor {
    fn lookup(functions: &SymbolTable, name: &IdTy) -> bool {
        functions.lookup_symbol(name)
    }

    fn visit(&self, functions: &SymbolTable) -> Result<bool>;
}