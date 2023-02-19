use crate::ast::*;
use crate::pass::Pass;
use anyhow::Result;

use crate::traversal::CodegenTraversal;
use crate::visitors::CodegenVisitorTrait;

use crate::codegen::Codegen;

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

    pub fn run_codegen(
        &mut self,
        visitor: &mut impl CodegenVisitorTrait,
        codegen: &mut Codegen,
        mut traversal: CodegenTraversal,
        program: &mut Program,
    ) -> Result<()> {
        let _ = traversal.traverse(visitor, program, codegen)?;

        Ok(())
    }
}
