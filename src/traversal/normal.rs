use crate::ast::*;
use crate::traversal::Traversal;
use crate::visitors::Visitor;
use anyhow::Result;

pub struct NormalTraversal;

impl Traversal for NormalTraversal {
    fn traverse(&mut self, visitor: &mut Box<dyn Visitor>, program: &mut Program) -> Result<()> {
        visitor.visit_program(program)?;

        for struc in program.structs.iter() {
            visitor.visit_struct(struc)?;
        }

        // Register all functions separetely
        // This is necessary, because functions need not to be defined before the caller.
        for function in program.functions.iter() {
            visitor.visit_func(function)?;
        }

        for function in program.functions.iter() {
            for statement in function.statements.iter() {
                let expr = statement.get_inner();

                if let Some(expr) = expr {
                    recur_expr(expr, visitor)?;
                    visitor.visit_expr(expr)?;
                }

                visitor.visit_statement(statement.as_ref())?;
            }
        }

        Ok(())
    }
}

fn recur_expr(expr: &Box<Expr>, visitor: &mut Box<dyn Visitor>) -> Result<()> {
    match expr.as_ref() {
        Expr::Id(_) => {}
        Expr::Num(_) => {}
        Expr::Struct(_) => {}
        Expr::Single(ref term) => {
            visitor.visit_term(term)?;
        }
        Expr::Dual(_, ref term1, ref term2) => {
            visitor.visit_term(term1)?;
            visitor.visit_term(term2)?;
        }
        Expr::Call(_, ref exprs) => {
            for argument in exprs {
                visitor.visit_expr(argument)?;
            }
        }
    }

    Ok(())
}
