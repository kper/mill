use crate::visitors::*; 
use crate::ast::*;

impl AbstractNode for Program {
    fn accept(&mut self, visitor: &mut impl Visitor) {
        visitor.visit_program(self);
    }
}

impl AbstractNode for Func {
    fn accept(&mut self, visitor: &mut impl Visitor) {
        visitor.visit_func(self);
    }
}

impl AbstractNode for Statement {
    fn accept(&mut self, visitor: &mut impl Visitor) {
        visitor.visit_statement(self);
    }
}

impl AbstractNode for Guard {
    fn accept(&mut self, visitor: &mut impl Visitor) {
        visitor.visit_guard(self);
    }
}

impl AbstractNode for Expr {
    fn accept(&mut self, visitor: &mut impl Visitor) {
        visitor.visit_expr(self);
    }
}

impl AbstractNode for Term {
    fn accept(&mut self, visitor: &mut impl Visitor) {
        visitor.visit_term(self);
    }
}

impl AbstractNode for Struct {
    fn accept(&mut self, visitor: &mut impl Visitor) {
        visitor.visit_struct(self);
    }
}