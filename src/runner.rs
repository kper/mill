use crate::pass::Pass;
use crate::ast::*;
use anyhow::Result;

use crate::visitors::CodegenVisitorTrait;
use crate::traversal::CodegenTraversal;

use inkwell::context::Context;

pub struct Runner;

impl Runner {
    /**
     * Run the visitors
     */
    pub fn run_visitors(&mut self, passes: &mut [Pass], program: &mut Program) -> Result<()> {

        for pass in passes.iter_mut() {
            pass.run(program)?;
        }

        
        Ok(())
    }

    pub fn run_codegen<T:  CodegenVisitorTrait>(&mut self, mut visitor: T, mut traversal: CodegenTraversal, program: &mut Program, context: &Context) -> Result<T> {
        let _ = traversal.traverse(&mut visitor, program, context)?;

        Ok(visitor)
    }
}