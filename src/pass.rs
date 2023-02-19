use crate::ast::*;
use crate::traversal::*;
use crate::visitors::*;
use anyhow::Result;

pub struct Pass {
    visitor: Box<dyn Visitor>,
    traversal: Box<dyn Traversal>,
}

impl Pass {
    pub fn new(visitor: Box<dyn Visitor>, traversal: Box<dyn Traversal>) -> Self {
        Self { visitor, traversal }
    }

    #[allow(dead_code)]
    pub fn get_name(&self) -> String {
        self.visitor.get_name().clone()
    }

    #[allow(dead_code)]
    pub fn get_visitor(&self) -> &Box<dyn Visitor> {
        &self.visitor
    }

    pub fn run(&mut self, program: &mut Program) -> Result<()> {
        self.traversal.traverse(&mut self.visitor, program)?;

        Ok(())
    }
}
