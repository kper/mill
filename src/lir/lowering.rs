use crate::ast::{Expr, Func, Program, Statement, Struct};
use anyhow::{bail, Context, Result};

use super::tree::*;

/**
 * Transforms a syntax tree into a lowered representation of the program.
 */
pub struct LoweringPass;

impl LoweringPass {
    pub fn lower(&mut self, program: &Program) -> Result<LoweredProgram> {
        let (functions, errors): (Vec<_>, Vec<_>) = program
            .functions
            .iter()
            .map(|x| self.map_function(x).context("Mapping failed"))
            .partition(Result::is_ok);

        if errors.len() > 0 {
            bail!("Lowering functions has failed {:?}", errors);
        }

        Ok(LoweredProgram::new(
            functions.into_iter().map(Result::unwrap).collect(),
        ))
    }

    fn map_function(&mut self, function: &Func) -> Result<LoweredFunction> {
        let mut blocks = vec![BasicBlock::empty()];
        for stmt in &function.statements {
            let current_block = blocks.last_mut().context("There is no basic block")?;

            let lowered_stmt = match stmt.as_ref() {
                Statement::Assign(id, ref value) => LoweredStatement::Assignment(
                    Variable::new(id.clone(), false),
                    self.map_expr(value)?,
                ),
                _ => panic!(),
            };

            current_block.add_to_bottom(lowered_stmt)?;
        }

        Ok(LoweredFunction {
            id: function.id.clone(),
            pars: function.pars.clone(),
            entry: Self::move_first_block(blocks),
        })
    }

    fn map_expr(&mut self, expr: &Box<Expr>) -> Result<LoweredExpression> {
        Ok(match *expr.as_ref() {
            Expr::Num(num) => LoweredExpression::Term(Term::I32(num.clone())),
            _ => unimplemented!(),
        })
    }

    fn move_first_block(mut blocks: Vec<BasicBlock>) -> BasicBlock {
        blocks.swap_remove(0)
    }

    fn map_struct(&mut self, struc: &Struct) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::Identifier;

    use super::*;

    fn create_identifier() -> Identifier {
        Identifier::new("test".to_string(), 0, 100, None)
    }

    #[test]
    fn lower_assignment() {
        let mut pass = LoweringPass;
        let instruction = Statement::Assign(create_identifier(), Box::new(Expr::Num(1)));
        let program = Program {
            structs: Vec::new(),
            functions: vec![Func {
                id: create_identifier(),
                pars: vec![create_identifier()],
                statements: vec![Box::new(instruction)],
                ret_ty: None,
            }],
        };

        // When
        let result = pass.lower(&program).unwrap();

        // Then
        assert_eq!(result.get_entries().len(), 1);
        assert_eq!(result.get_entries().get(0).unwrap().id.get_name(), "test");
        assert_eq!(result.get_entries().get(0).unwrap().pars.len(), 1);
        assert_eq!(
            result.get_entries().get(0).unwrap().entry.get_statements(),
            vec![LoweredStatement::Assignment(
                Variable::new(create_identifier(), false),
                LoweredExpression::Term(Term::I32(1))
            )]
        );
    }
}
