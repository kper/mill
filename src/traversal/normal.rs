use crate::traversal::Traversal;
use crate::ast::*;
use anyhow::Result;
use crate::visitors::Visitor;
use either::Either;

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

                    let expr_or_guard = statement.get_inner();

                    if let Some(expr_or_guard) = expr_or_guard {
                        match expr_or_guard {
                            Either::Left(expr) => {
                                recur_expr(expr, visitor)?;

                                visitor.visit_expr(expr)?;
                            }
                            Either::Right(guards) => {
                                for guard in guards {
                                    visitor.visit_guard(guard)?;
                                } 
                            }
                        }
                    }

                    visitor.visit_statement(statement.as_ref())?;
                    
                }
            }

        Ok(())
    }
}

fn recur_expr(expr: &Box<Expr>, visitor: &mut Box<dyn Visitor>) -> Result<()> {

    match expr.as_ref() {
        Expr::Id(_) => {},
        Expr::Num(_) => {},
        Expr::Struct(_) => {}
        Expr::Single(ref term) => {
            visitor.visit_term(term)?;
        }
        Expr::Dual(_, ref term1, ref term2) => {
            visitor.visit_term(term1)?;
            visitor.visit_term(term2)?;
        }
        Expr::Chained(_, ref term, expr) => {
            visitor.visit_term(term)?;
            recur_expr(expr, visitor)?;
        }
        Expr::Unchained(_, ref term) => {
            visitor.visit_term(term)?;
        }
    }

    Ok(())
}