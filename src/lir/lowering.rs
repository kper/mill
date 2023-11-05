use crate::ast::{Expr, Func, Program, Statement, Struct, Term};
use anyhow::{bail, Context, Result};
use log::info;

use super::tree::*;

/**
 * Transforms a syntax tree into a lowered representation of the program.
 */
pub struct LoweringPass {
    basic_block_counter: BasicBlockId,
}

impl LoweringPass {
    pub fn default() -> Self {
        Self {
            basic_block_counter: BasicBlockId::default(),
        }
    }

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
            "Program".to_string(),
            functions.into_iter().map(Result::unwrap).collect(),
        ))
    }

    fn map_function(&mut self, function: &Func) -> Result<LoweredFunction> {
        let mut blocks = vec![BasicBlock::empty(
            self.basic_block_counter.fetch_and_increment(),
        )];
        for stmt in &function.statements {
            let current_block = blocks.last_mut().context("There is no basic block")?;

            if let Some(next_block) = self.map_block(stmt, current_block)? {
                current_block.add_successor(next_block.get_id());
                blocks.push(next_block);
            }
        }

        Ok(LoweredFunction {
            id: function.id.clone(),
            pars: function.pars.clone(),
            entry: blocks
                .first()
                .context("Function has no blocks")?
                .get_id()
                .clone(),
            blocks: blocks,
        })
    }

    fn map_stmts(
        &mut self,
        stmts: &[Box<Statement>],
        current_block: &mut BasicBlock,
    ) -> Result<()> {
        for stmt in stmts {
            self.map_block(stmt, current_block)?;
        }

        Ok(())
    }

    /// Maps a block and returns a BasicBlock if a new block has been created and should be continued.
    fn map_block(
        &mut self,
        stmt: &Box<Statement>,
        current_block: &mut BasicBlock,
    ) -> Result<Option<BasicBlock>> {
        match stmt.as_ref() {
            Statement::Definition(a, b) => {
                let stmt = LoweredStatement::Definition(
                    Variable::new(a.clone(), false),
                    self.map_expr(b)?,
                );
                current_block.add_to_bottom(stmt)?;
            }
            Statement::Assign(id, ref value) => {
                let stmt = LoweredStatement::Assignment(
                    Variable::new(id.clone(), false),
                    self.map_expr(value)?,
                );

                current_block.add_to_bottom(stmt)?;
            }
            Statement::RetVoid => current_block.add_to_bottom(LoweredStatement::RetVoid)?,
            Statement::Conditional(condition, statements) => {
                let mut if_block =
                    BasicBlock::empty(self.basic_block_counter.fetch_and_increment());
                let then_block = BasicBlock::empty(self.basic_block_counter.fetch_and_increment());

                self.map_stmts(statements, &mut if_block)?;
                let stmt = LoweredStatement::ConditionalJump(
                    self.map_expr(condition)?,
                    Box::new(if_block),
                );

                current_block.add_to_bottom(stmt)?;
                return Ok(Some(then_block));
            }
            _ => panic!(),
        };

        Ok(None)
    }

    fn map_expr(&mut self, expr: &Box<Expr>) -> Result<LoweredExpression> {
        Ok(match *expr.as_ref() {
            Expr::Term(ref term) => {
                LoweredExpression::Term(self.map_term(term).context("Cannot map the term")?)
            }
            Expr::Binary(ref op, ref a, ref b) => LoweredExpression::Binary(
                op.clone(),
                self.map_term(a).context("Cannot map the term")?,
                self.map_term(b).context("Cannot map the term")?,
            ),
            Expr::Call(ref function_name, ref parameters) => LoweredExpression::Call(
                function_name.clone(),
                parameters
                    .iter()
                    .map(|x| Box::new(self.map_expr(x).unwrap()))
                    .collect::<Vec<_>>(),
            ),
            Expr::Struct(_) => unimplemented!(),
        })
    }

    fn map_term(&mut self, term: &Box<Term>) -> Result<LoweredTerm> {
        Ok(match *term.as_ref() {
            Term::Num(num) => LoweredTerm::Constant(num),
            Term::Id(ref id) => LoweredTerm::Id(Variable::new(id.clone(), false)),
        })
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
        let mut pass = LoweringPass::default();
        let instruction = Statement::Assign(
            create_identifier(),
            Box::new(Expr::Term(Box::new(Term::Num(1)))),
        );
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
            result.get_entries().get(0).unwrap().entry,
            BasicBlockId::default()
        );
        assert_eq!(
            result
                .get_entries()
                .get(0)
                .unwrap()
                .blocks
                .get(0)
                .unwrap()
                .get_statements(),
            vec![LoweredStatement::Assignment(
                Variable::new(create_identifier(), false),
                LoweredExpression::Term(LoweredTerm::Constant(1))
            )]
        );
    }
}
