use crate::ast::*;
use crate::traversal::Traversal;
use crate::visitors::Visitor;
use crate::visitors::Ctx;
use anyhow::Result;

pub struct NormalTraversal;

impl Traversal for NormalTraversal {
    fn traverse(&mut self, visitor: &mut Box<dyn Visitor>, program: &mut Program) -> Result<()> {
        let ctx = Ctx::empty();
        visitor.visit_program(ctx.clone(), program)?;

        for struc in program.structs.iter() {
            visitor.visit_struct(ctx.clone(), struc)?;
        }

        // Register all functions separately
        // This is necessary because functions need not to be defined before the caller.
        for function in program.functions.iter() {
            visitor.visit_func(ctx.clone(), function)?;
        }

        for function in program.functions.iter() {
            for statement in function.statements.iter() {
                let expr = statement.get_inner();
                let ctx = Ctx { function: Some(function)};

                if let Some(expr) = expr {
                    recur_expr(ctx.clone(), expr, visitor)?;
                    visitor.visit_expr(ctx.clone(), expr)?;
                }

                visitor.visit_statement(ctx.clone(), statement.as_ref())?;
            }
        }

        Ok(())
    }
}

fn recur_expr<'a>(ctx: Ctx<'a>, expr: &Box<Expr>, visitor: &mut Box<dyn Visitor>) -> Result<()> {
    match expr.as_ref() {
        Expr::Id(_) => {}
        Expr::Num(_) => {}
        Expr::Struct(_) => {}
        Expr::Single(ref term) => {
            visitor.visit_term(ctx, term)?;
        }
        Expr::Dual(_, ref term1, ref term2) => {
            visitor.visit_term(ctx.clone(), term1)?;
            visitor.visit_term(ctx, term2)?;
        }
        Expr::Call(_, ref exprs) => {
            for argument in exprs {
                visitor.visit_expr(ctx.clone(), argument)?;
            }
        }
    }

    Ok(())
}
