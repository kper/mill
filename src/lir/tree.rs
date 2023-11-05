use crate::ast::{Identifier, Opcode};
use anyhow::Result;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BasicBlockId(usize);

impl BasicBlockId {
    pub fn default() -> Self {
        BasicBlockId(0)
    }

    pub fn get_value(&self) -> usize {
        self.0
    }

    pub fn fetch_and_increment(&mut self) -> BasicBlockId {
        let current = self.0;
        self.0 += 1;

        BasicBlockId(current)
    }
}

#[derive(Debug)]
pub struct LoweredProgram {
    name: String,
    entries: Vec<LoweredFunction>,
}

#[derive(Debug)]
pub struct LoweredFunction {
    pub id: Identifier,
    pub pars: Vec<Identifier>,
    pub entry: BasicBlockId,
    pub blocks: Vec<BasicBlock>,
}

impl LoweredProgram {
    pub fn new(name: String, entries: Vec<LoweredFunction>) -> Self {
        Self { name, entries }
    }

    pub(crate) fn get_name(&self) -> &String {
        &self.name
    }

    pub(crate) fn get_entries(&self) -> &[LoweredFunction] {
        &self.entries
    }
}

impl LoweredFunction {
    pub fn new(id: Identifier, pars: Vec<Identifier>, entry: BasicBlockId, blocks: Vec<BasicBlock>) -> Self {
        Self { id, pars, entry, blocks }
    }
}

#[derive(Debug, PartialEq)]
pub struct BasicBlock {
    id: BasicBlockId,
    statements: Vec<LoweredStatement>,
    next: Vec<BasicBlockId>,
}

impl BasicBlock {
    pub fn empty(id: BasicBlockId) -> Self {
        Self {
            id,
            statements: Vec::new(),
            next: Vec::new(),
        }
    }

    pub(crate) fn get_id(&self) -> &BasicBlockId {
        &self.id
    }

    pub(crate) fn get_statements(&self) -> &[LoweredStatement] {
        &self.statements
    }

    pub(crate) fn get_next(&self) -> &[BasicBlockId] {
        &self.next
    }

    pub fn add_to_bottom(&mut self, stmt: LoweredStatement) -> Result<()> {
        self.statements.push(stmt);
        Ok(())
    }

    /// Every basic block can have multiple successors.
    /// This method adds the given basic block as successor of the current basic block.
    pub fn add_successor(&mut self, succ: &BasicBlockId) {
        self.next.push(*succ);
    }


}

#[derive(Debug, PartialEq)]
pub struct Variable {
    ident: Identifier,
    /// Was this variable generated by the compiler
    generated: bool,
}

impl Variable {
    pub fn new(ident: Identifier, generated: bool) -> Self {
        Self { ident, generated }
    }

    pub fn get_ident(&self) -> &Identifier {
        &self.ident
    }
}

#[derive(Debug, PartialEq)]
pub enum LoweredExpression {
    Term(LoweredTerm),
    Binary(Opcode, LoweredTerm, LoweredTerm),
}

#[derive(Debug, PartialEq)]
pub enum LoweredTerm {
    Constant(i64),
}

#[derive(Debug, PartialEq)]
pub enum LoweredStatement {
    Definition(Variable, LoweredExpression),
    Assignment(Variable, LoweredExpression),
    Phi(Variable, Vec<Variable>),
    Unconditional_Jump(Box<BasicBlock>),
    Conditional_Jump(LoweredExpression, Box<BasicBlock>),
    RetVoid,
}
