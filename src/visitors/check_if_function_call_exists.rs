use crate::visitors::*;
use crate::ast::*;

use crate::symbol_table::SymbolTable;

use anyhow::bail;

#[derive(Debug, Default)]
pub struct CheckIfFunctionCallExistsVisitor {
    symbol_table: SymbolTable
}

impl Visitor for CheckIfFunctionCallExistsVisitor {
    fn visit_program<'ctx>(&mut self, program: &'ctx Program) -> Result<()> {
        //println!("Visiting program");
        Ok(())
    }

    fn visit_func<'ctx>(&mut self, func: &'ctx Func) -> Result<()> {
        //println!("Visiting func");
        Ok(())
    }

    fn visit_statement<'ctx>(&mut self, stmt: &'ctx Statement) -> Result<()> {
        //println!("Visiting statement");
        Ok(())
    }
    
    //fn visit_guard(&mut self, label: &Option<IdTy>, guard: &mut Guard) -> Result<()>;

    fn visit_guard<'ctx>(&mut self, guard: &'ctx Guard) -> Result<()> {
        //println!("Visiting guard");
        Ok(())
    }

    fn visit_expr<'ctx>(&mut self, expr: &'ctx Expr) -> Result<()> {
        //println!("Visiting expr");
        Ok(())
    }

    fn visit_term<'ctx>(&mut self, term: &'ctx Term) -> Result<()> {
        let symbol_table = &self.symbol_table;
        match term {
            Term::Call(id, exprs) => {
                if !symbol_table.lookup_symbol(&id.get_name()) {
                    bail!("Function {} is not defined", id);
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn visit_struct<'ctx>(&mut self, stru: &'ctx Struct) -> Result<()> {
        //println!("Visiting struct");
        Ok(())
    }
}