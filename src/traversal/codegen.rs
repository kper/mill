use crate::traversal::Traversal;
use crate::ast::*;
use anyhow::Result;
use crate::visitors::CodegenVisitorTrait;
use either::Either;

use inkwell::context::Context;
use crate::codegen::Codegen;

pub struct CodegenTraversal;

impl CodegenTraversal {
    pub fn traverse<'a>(&mut self, visitor: &'a mut impl CodegenVisitorTrait, program: &mut Program, context: &'a Context) -> Result<Codegen<'a>> {

           let mut codegen = Codegen::new(&context);

           visitor.visit_program(program, &mut codegen, &context)?;

            for struc in program.structs.iter() {
                visitor.visit_struct(struc, &mut codegen, &context)?;
            }

            // Register all functions separetely
            // This is necessary, because functions need not to be defined before the caller.
            for function in program.functions.iter() {
                visitor.visit_func(function, &mut codegen, &context)?;
            }

            
            for function in program.functions.iter() {
                for statement in function.statements.iter() {

                    let expr_or_guard = statement.get_inner();

                    if let Some(expr_or_guard) = expr_or_guard {
                        match expr_or_guard {
                            Either::Left(expr) => {
                                recur_expr(expr, visitor, &mut codegen, &context)?;

                                visitor.visit_expr(expr, &mut codegen, &context)?;
                            }
                            Either::Right(guards) => {
                                for guard in guards {
                                    visitor.visit_guard(guard, &mut codegen, &context)?;
                                } 
                            }
                        }
                    }

                    visitor.visit_statement(statement.as_ref(), &mut codegen, &context)?;
                }
            }

        Ok(codegen)
    }
}

fn recur_expr<'a>(expr: &Box<Expr>, visitor: &'a mut impl CodegenVisitorTrait, codegen: &'a mut Codegen<'a>, context: &'a Context) -> Result<()> {

    match expr.as_ref() {
        Expr::Id(_) => {},
        Expr::Num(_) => {},
        Expr::Struct(_) => {}
        Expr::Single(ref term) => {
            visitor.visit_term(term, codegen, context)?;
        }
        Expr::Dual(_, ref term1, ref term2) => {
            visitor.visit_term(term1, codegen, context)?;
            visitor.visit_term(term2, codegen, context)?;
        }
        Expr::Chained(_, ref term, expr) => {
            visitor.visit_term(term, codegen, context)?;
            recur_expr(expr, visitor, codegen, context)?;
        }
        Expr::Unchained(_, ref term) => {
            visitor.visit_term(term, codegen, context)?;
        }
    }

    Ok(())
}