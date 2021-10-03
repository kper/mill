use crate::traversal::Traversal;
use crate::ast::*;
use anyhow::Result;
use crate::visitors::CodegenVisitorTrait;
use either::Either;

use inkwell::context::Context;
use crate::codegen::Codegen;

pub struct CodegenTraversal;

impl CodegenTraversal {
    pub fn traverse<'a>(&mut self, visitor: &'a mut impl CodegenVisitorTrait<'a>, program: &mut Program, codegen: &'a mut Codegen<'a>) -> Result<()> {

           //let mut codegen = Codegen::new(&context);

            visitor.visit_program(program, codegen)?;

            for struc in program.structs.iter() {
                visitor.visit_struct(struc, codegen)?;
            }

            // Register all functions separetely
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

fn recur_expr<'a>(expr: &Box<Expr>, visitor: &mut impl CodegenVisitorTrait<'a>, codegen: &mut Codegen<'a>) -> Result<()> {

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
        Expr::Chained(_, ref term, expr) => {
            visitor.visit_term(term, codegen)?;
            recur_expr(expr, visitor, codegen)?;
        }
        Expr::Unchained(_, ref term) => {
            visitor.visit_term(term, codegen)?;
        }
    }

    Ok(())
}