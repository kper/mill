use crate::ast::*;
use crate::visitors::*;
use crate::traversal::*;
use anyhow::Result;

pub struct Pass {
    visitor: Box<dyn Visitor>,
    traversal: Box<dyn Traversal>,
}

impl Pass
{
    pub fn new(visitor: Box<dyn Visitor>, traversal: Box<dyn Traversal>) -> Self {
        Self {
            visitor,
            traversal
        }
    }

    pub fn run(&mut self, program: &mut Program,) -> Result<()> {
        self.traversal.traverse(&mut self.visitor, program)?;

        Ok(())
    }
}