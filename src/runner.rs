use crate::ast::*;
use anyhow::{Result, Context};

pub struct Runner;

impl Runner {
    pub fn lowering(&mut self, program: &Program) -> Result<crate::lir::tree::LoweredProgram> {
        let mut pass = crate::lir::lowering::LoweringPass;
        pass.lower(program).context("Lowering failed")
    }

    /*
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
    */
}
