use crate::ast::*;
use crate::visitors::*;
use anyhow::Result;



pub trait AbstractNode {
    fn accept<'a>(&mut self, ctx: Ctx<'a>, visitor: &mut impl Visitor) -> Result<()>;
}

impl AbstractNode for Program {
    fn accept<'a>(&mut self, ctx: Ctx<'a>, visitor: &mut impl Visitor) -> Result<()> {
        visitor.visit_program(ctx, self)
    }
}

impl AbstractNode for Func {
    fn accept<'a>(&mut self, ctx: Ctx<'a>, visitor: &mut impl Visitor) -> Result<()> {
        visitor.visit_func(ctx, self)
    }
}

impl AbstractNode for Statement {
    fn accept<'a>(&mut self, ctx: Ctx<'a>, visitor: &mut impl Visitor) -> Result<()> {
        visitor.visit_statement(ctx, self)
    }
}

impl AbstractNode for Expr {
    fn accept<'a>(&mut self, ctx: Ctx<'a>, visitor: &mut impl Visitor) -> Result<()> {
        visitor.visit_expr(ctx, self)
    }
}

impl AbstractNode for Term {
    fn accept<'a>(&mut self, ctx: Ctx<'a>, visitor: &mut impl Visitor) -> Result<()> {
        visitor.visit_term(ctx, self)
    }
}

impl AbstractNode for Struct {
    fn accept<'a>(&mut self, ctx: Ctx<'a>, visitor: &mut impl Visitor) -> Result<()> {
        visitor.visit_struct(ctx,self)
    }
}
