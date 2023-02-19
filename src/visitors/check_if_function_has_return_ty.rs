use crate::visitors::*;

use anyhow::{bail};

#[derive(Debug, Default)]
pub struct CheckIfFunctionHasReturnTyVisitor;

impl Visitor for CheckIfFunctionHasReturnTyVisitor {
    fn get_name(&self) -> String {
        "CheckIfFunctionHasReturnTyVisitor".to_string()
    }

    fn visit_program<'a>(&mut self, _ctx: Ctx<'a>, _program: &Program) -> Result<()> {
        Ok(())
    }

    fn visit_func<'a>(&mut self, _ctx: Ctx<'a>, _func: &Func) -> Result<()> {
        Ok(())
    }

    fn visit_statement<'a>(&mut self, ctx: Ctx<'a>, stmt: &Statement) -> Result<()> {
        if let Some(function) = ctx.function {
            match stmt {
                Statement::RetVoid => {
                    if function.ret_ty.is_some() {
                        bail!("Declared function's return type is not void. But the return statement does not return anything.");
                    }
                }
                Statement::Ret(_exprs) => {
                    // TODO add type checks if the ret ty is correct.
                    if function.ret_ty.is_none() {
                        bail!("Declared function's return type is void. But a variable is returned.");
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn visit_expr<'a>(&mut self, _ctx: Ctx<'a>, _expr: &Expr) -> Result<()> {
        Ok(())
    }

    fn visit_term<'a>(&mut self, _ctx: Ctx<'a>, _term: &'a Term) -> Result<()> {
        Ok(())
    }

    fn visit_struct<'a>(&mut self, _ctx: Ctx<'a>, _stru: &'a Struct) -> Result<()> {
        Ok(())
    }
}
