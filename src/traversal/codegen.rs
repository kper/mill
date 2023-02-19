use crate::ast::*;
use crate::visitors::CodegenVisitorTrait;
use anyhow::Result;

use crate::codegen::Codegen;

pub struct CodegenTraversal;

impl CodegenTraversal {
    pub fn traverse(
        &mut self,
        visitor: &mut impl CodegenVisitorTrait,
        program: &mut Program,
        codegen: &mut Codegen,
    ) -> Result<()> {
        visitor.visit_program(program, codegen)?;

        for struc in program.structs.iter() {
            visitor.visit_struct(struc, codegen)?;
        }

        // Register all functions separately
        // This is necessary, because functions need not to be defined before the caller.
        for function in program.functions.iter() {
            visitor.visit_func(function, codegen)?;
        }

        for function in program.functions.iter() {
            visitor.set_block_position_to_function(function, codegen)?;

            for param in function.pars.iter() {
                visitor.visit_param(function, param, codegen)?;
            }

            for statement in function.statements.iter() {
                visitor.visit_statement(function, statement.as_ref(), codegen)?;
            }
        }

        Ok(())
    }
}
