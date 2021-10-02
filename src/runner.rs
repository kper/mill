use crate::pass::Pass;
use crate::ast::*;
use anyhow::Result;

use crate::visitors::CodegenVisitorTrait;
use crate::traversal::CodegenTraversal;

pub struct Runner;

impl Runner {
    /**
     * Run the visitors
     */
    pub fn run_visitors(&mut self, passes: &mut [Pass], program: &mut Program) -> Result<()> {

        for mut pass in passes.iter_mut() {
            pass.run(program)?;
        }

        
        Ok(())
    }

    pub fn run_codegen<T:  CodegenVisitorTrait>(&mut self, mut visitor: T, mut traversal: CodegenTraversal, program: &mut Program) -> Result<T> {
        traversal.traverse(&mut visitor, program)?;

        Ok(visitor)
    }
}