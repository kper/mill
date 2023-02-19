use crate::ast::*;
use crate::visitors::Visitor;
use anyhow::Result;

mod codegen;
mod normal;

pub use crate::traversal::codegen::*;
pub use crate::traversal::normal::*;

pub trait Traversal {
    fn traverse(&mut self, visitor: &mut Box<dyn Visitor>, program: &mut Program) -> Result<()>;
}
