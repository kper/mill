use crate::traversal::Traversal;
use crate::ast::*;
use anyhow::Result;
use crate::visitors::CodegenVisitorTrait;
use either::Either;

use crate::codegen::Codegen;

pub struct CodegenTraversal;

impl CodegenTraversal {
    pub fn traverse(&mut self, visitor: &mut impl CodegenVisitorTrait, program: &mut Program, codegen: &mut Codegen) -> Result<()> {
            visitor.visit_program(program, codegen)?;

            for struc in program.structs.iter() {
                visitor.visit_struct(struc, codegen)?;
            }

            // Register all functions separately
            // This is necessary, because functions need not to be defined before the caller.
            for function in program.functions.iter() {
                visitor.visit_func(function, codegen)?;
            }

            for function in program.functions.iter() {
                for statement in function.statements.iter() {

                    let expr_or_guard = statement.get_inner();

                    if let Some(expr_or_guard) = expr_or_guard {
                        match expr_or_guard {
                            Either::Left(expr) => {
                                recur_expr(expr, visitor, codegen)?;

                                visitor.visit_expr(expr, codegen)?;
                            }
                            Either::Right(guards) => {
                                for guard in guards {
                                    visitor.visit_guard(guard, codegen)?;
                                } 
                            }
                        }
                    }

                    visitor.visit_statement(statement.as_ref(),  codegen)?;
                }
            }

        Ok(())
    }
}

fn recur_expr(expr: &Box<Expr>, visitor: &mut impl CodegenVisitorTrait, codegen: &mut Codegen) -> Result<()> {

    match expr.as_ref() {
        Expr::Id(_) => {},
        Expr::Num(_) => {},
        Expr::Struct(_) => {}
        Expr::Single(ref term) => {
            visitor.visit_term(term, codegen)?;
        }
        Expr::Dual(_, ref term1, ref term2) => {
            visitor.visit_term(term1, codegen)?;
            visitor.visit_term(term2, codegen)?;
        }
    }

    Ok(())
}