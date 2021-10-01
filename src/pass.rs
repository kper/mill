use crate::ast::*;
use crate::visitors::*;
use crate::traversal::*;
use anyhow::Result;
use std::any::Any;

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

    pub fn get_name(&self) -> String {
        self.visitor.get_name().clone()
    }

    pub fn get_visitor(&self) -> &Box<dyn Visitor> {
        &self.visitor
    }

    /* 
    pub fn get_visitor_as_any(&self) -> Box<dyn Any> {
        let x = self.visitor.as_ref();
        Box::new(x)
    }*/

    pub fn run(&mut self, program: &mut Program,) -> Result<()> {
        self.traversal.traverse(&mut self.visitor, program)?;

        Ok(())
    }
}