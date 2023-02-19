use crate::visitors::*;

use crate::symbol_table::SymbolTable;

use anyhow::bail;
use log::debug;

/**
Check if a called function exists and if a function is only defined once.
*/
#[derive(Debug, Default)]
pub struct CheckIfFunctionCallExistsVisitor {
    symbol_table: SymbolTable
}

impl Visitor for CheckIfFunctionCallExistsVisitor {
    fn get_name(&self) -> String {
        "CheckIfFunctionCallExistsVisitor".to_string()
    }

    fn visit_program(&mut self, _program: &Program) -> Result<()> {
        Ok(())
    }

    fn visit_func(&mut self, func: &Func) -> Result<()> {
        debug!("{}: Calling `visit_func` for function: {}", self.get_name(), func.id.get_name());

        let symbol_table = &mut self.symbol_table;

        if symbol_table.lookup_symbol(&func.id.get_name()) {
            bail!("Function {} is already defined", func.id.get_name());
        }
        else {
            symbol_table.insert(&func.id.get_name())?;
        }

        Ok(())
    }

    fn visit_statement(&mut self, _stmt: &Statement) -> Result<()> {
        //println!("Visiting statement");
        Ok(())
    }

    fn visit_expr(&mut self, expr: &Expr) -> Result<()> {
        let symbol_table = &self.symbol_table;
        match expr {
            Expr::Call(id, _exprs) => {
                debug!("{}: Calling `visit_expr` for calling function: {}", self.get_name(), id);

                // It is also possible that another function call is nested in `_exprs`
                // but we can ignore it here, because it is the responsibility of the
                // Traversal to ensure the visiting.

                if !symbol_table.lookup_symbol(&id.get_name()) {
                    bail!("Function {} is not defined", id);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn visit_term<'ctx>(&mut self, _term: &'ctx Term) -> Result<()> {
        Ok(())
    }

    fn visit_struct<'ctx>(&mut self, _stru: &'ctx Struct) -> Result<()> {
        Ok(())
    }
}