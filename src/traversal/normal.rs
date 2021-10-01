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
            
            for function in program.functions.iter() {
                visitor.visit_func(function)?;

                for statement in function.statements.iter() {

                    let expr_or_guard = statement.get_inner();

                    if let Some(expr_or_guard) = expr_or_guard {
                        match expr_or_guard {
                            Either::Left(expr) => {
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