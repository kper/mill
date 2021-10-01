use crate::visitors::*;
use crate::ast::*;

use crate::symbol_table::SymbolTable;

use anyhow::bail;
use log::debug;
use std::any::Any;

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

    fn visit_program<'ctx>(&mut self, _program: &'ctx Program) -> Result<()> {
        Ok(())
    }

    fn visit_func<'ctx>(&mut self, func: &'ctx Func) -> Result<()> {
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

    fn visit_statement<'ctx>(&mut self, _stmt: &'ctx Statement) -> Result<()> {
        //println!("Visiting statement");
        Ok(())
    }
    
    //fn visit_guard(&mut self, label: &Option<IdTy>, guard: &mut Guard) -> Result<()>;

    fn visit_guard<'ctx>(&mut self, _guard: &'ctx Guard) -> Result<()> {
        //println!("Visiting guard");
        Ok(())
    }

    fn visit_expr<'ctx>(&mut self, _expr: &'ctx Expr) -> Result<()> {
        //println!("Visiting expr");
        Ok(())
    }

    fn visit_term<'ctx>(&mut self, term: &'ctx Term) -> Result<()> {
        let symbol_table = &self.symbol_table;
        match term {
            Term::Call(id, _exprs) => {
                debug!("{}: Calling `visit_term` for calling function: {}", self.get_name(), id);

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

    fn visit_struct<'ctx>(&mut self, _stru: &'ctx Struct) -> Result<()> {
        //println!("Visiting struct");
        Ok(())
    }
}