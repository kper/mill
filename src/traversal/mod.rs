use anyhow::Result;
use crate::visitors::Visitor;
use crate::ast::*;

mod normal;
mod codegen;

pub use crate::traversal::normal::*;
pub use crate::traversal::codegen::*;

pub trait Traversal {
    fn traverse(&mut self, visitor: &mut Box<dyn Visitor>, program: &mut Program) -> Result<()>;
}

