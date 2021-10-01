use crate::visitors::*; 
use crate::ast::*;

impl<'ctx> AbstractNode<'ctx> for Program {
    fn accept(&'ctx mut self, visitor: &mut impl Visitor<'ctx>) {
        visitor.visit_program(self);
    }
}

impl<'ctx> AbstractNode<'ctx> for Func {
    fn accept(&'ctx mut self, visitor: &mut impl Visitor<'ctx>) {
        visitor.visit_func(self);
    }
}

impl<'ctx> AbstractNode<'ctx> for Statement {
    fn accept(&'ctx mut self, visitor: &mut impl Visitor<'ctx>) {
        visitor.visit_statement(self);
    }
}

impl<'ctx> AbstractNode<'ctx> for Guard {
    fn accept(&'ctx mut self, visitor: &mut impl Visitor<'ctx>) {
        visitor.visit_guard(self);
    }
}

impl<'ctx> AbstractNode<'ctx> for Expr {
    fn accept(&'ctx mut self, visitor: &mut impl Visitor<'ctx>) {
        visitor.visit_expr(self);
    }
}

impl<'ctx> AbstractNode<'ctx> for Term {
    fn accept(&'ctx mut self, visitor: &mut impl Visitor<'ctx>) {
        visitor.visit_term(self);
    }
}

impl<'ctx> AbstractNode<'ctx> for Struct {
    fn accept(&'ctx mut self, visitor: &mut impl Visitor<'ctx>) {
        visitor.visit_struct(self);
    }
}