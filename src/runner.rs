use crate::pass::Pass;
use crate::ast::*;
use anyhow::Result;

use crate::visitors::CodegenVisitorTrait;
use crate::traversal::CodegenTraversal;

use crate::codegen::Codegen;

use inkwell::context::Context;

use crate::Arena;

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

    pub fn run_codegen<'ctx, T: 'ctx + CodegenVisitorTrait<'ctx>>(&mut self, mut arena: Arena<'ctx, T>, mut traversal: CodegenTraversal, program: &mut Program) -> Result<()> {
        let visitor = &mut arena.visitor;
        let codegen = &mut arena.codegen;

        let _ = traversal.traverse(visitor, program, codegen)?;

        Ok(())
    }
}