use anyhow::Result;
use crate::visitors::Visitor;
use crate::ast::*;

mod normal;

pub use crate::traversal::normal::*;

pub trait Traversal {
    fn traverse(&mut self, visitor: &mut Box<dyn Visitor>, program: &mut Program) -> Result<()>;
}

