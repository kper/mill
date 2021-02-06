use crate::codegen::Codegen;
use crate::symbol_table::SymbolTable;
use crate::visitors::CheckIfFunctionCallExistsVisitor;
use crate::visitors::CodegenVisitor;
use anyhow::{anyhow, bail, Context, Result};
use std::collections::HashSet;

pub type IdTy = String;

/*
#[derive(Debug, Error)]
pub enum Error {
    #[error("The symbol `{0}` is already defined.")]
    SymbolAlreadyDefined(IdTy),
    #[error("The symbol `{0}` is not defined.")]
    SymbolNotDefined(IdTy),
    #[error("The function `{0}` is not defined.")]
    FunctionNotDefined(IdTy),
    #[error(transparent)]
    Other(#[from] anyhow::Error)
}*/

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Func>,
}

impl Program {
    /// Get all function names
    pub fn get_function_names(&self) -> Result<SymbolTable> {
        let mut set = SymbolTable::default();

        for name in self.functions.iter().map(|w| &w.id) {
            set.insert(name)?;
        }

        Ok(set)
    }

    /// Check if any functions has a duplicated name
    /// Returns `true` if the name already exists. Otherwise return `false`.
    pub fn check_duplicated_names(&self) -> bool {
        let mut set = HashSet::new();

        for name in self.functions.iter().map(|w| &w.id) {
            if set.contains(&name) {
                return true;
            }

            set.insert(name);
        }

        return false;
    }

    pub fn codegen_to_file(&mut self, path: &str) -> Result<()> {
        use inkwell::context::Context;

        let context = Context::create();
        let module = context.create_module("main");

        let mut codegen = Codegen::new(&context, module);

        codegen.visit_program(self)?;

        codegen.write_bitcode(path)?;

        Ok(())
    }

    pub fn codegen_to_ir(&mut self) -> Result<String> {
        use inkwell::context::Context;

        let context = Context::create();
        let module = context.create_module("main");

        let mut codegen = Codegen::new(&context, module);

        codegen.visit_program(self)?;

        Ok(codegen.get_ir())
    }
}

impl CheckIfFunctionCallExistsVisitor for Program {
    fn visit(&self, symbol_table: &SymbolTable) -> Result<bool> {
        for function in &self.functions {
            <Func as CheckIfFunctionCallExistsVisitor>::visit(function, symbol_table)?;
        }

        Ok(true)
    }
}

#[derive(Debug)]
pub struct Func {
    pub id: IdTy,
    pub pars: Vec<IdTy>,
    pub statements: Vec<Box<Statement>>,
    /// Symbol table for xxx
    symbol_table: SymbolTable,
}

impl Func {
    pub fn new(id: IdTy, pars: Vec<IdTy>, statements: Vec<Box<Statement>>) -> Result<Self> {
        let mut symbol_table = SymbolTable::default();

        for stmt in statements.iter() {
            match &**stmt {
                Statement::Assign(id, _) => symbol_table.insert(id)?,
                Statement::ReAssign(id, _) => {
                    if !symbol_table.lookup_symbol(&id) {
                        bail!("Symbol {} is not defined", id);
                    }
                }
                _ => {}
            }
        }

        Ok(Self {
            id,
            pars,
            statements,
            symbol_table,
        })
    }
}

impl CheckIfFunctionCallExistsVisitor for Func {
    fn visit(&self, symbol_table: &SymbolTable) -> Result<bool> {
        for stmt in &self.statements {
            <Statement as CheckIfFunctionCallExistsVisitor>::visit(stmt, symbol_table)?;
        }

        Ok(true)
    }
}

#[derive(Debug)]
pub enum Statement {
    Ret(Box<Expr>),
    Assign(IdTy, Box<Expr>),
    ReAssign(IdTy, Box<Expr>),
    Conditional(Option<IdTy>, Vec<Box<Guard>>),
}

impl CheckIfFunctionCallExistsVisitor for Statement {
    fn visit(&self, symbol_table: &SymbolTable) -> Result<bool> {
        match &*self {
            Statement::Ret(expr) => {
                <Expr as CheckIfFunctionCallExistsVisitor>::visit(&expr, symbol_table)?;
            }
            Statement::Assign(_, expr) => {
                <Expr as CheckIfFunctionCallExistsVisitor>::visit(&expr, symbol_table)?;
            }
            Statement::ReAssign(_, expr) => {
                <Expr as CheckIfFunctionCallExistsVisitor>::visit(&expr, symbol_table)?;
            }
            Statement::Conditional(_, guards) => {
                for guard in guards {
                    <Guard as CheckIfFunctionCallExistsVisitor>::visit(&guard, symbol_table)?;
                }
            }
        }

        Ok(true)
    }
}

#[derive(Debug)]
pub struct Guard {
    pub guard: Option<Box<Expr>>,
    pub statements: Vec<Box<Statement>>,
    pub continuation: Continuation,
}

impl CheckIfFunctionCallExistsVisitor for Guard {
    fn visit(&self, symbol_table: &SymbolTable) -> Result<bool> {
        if let Some(expr) = &self.guard {
            <Expr as CheckIfFunctionCallExistsVisitor>::visit(&expr, symbol_table)?;
        }

        for stmt in &self.statements {
            <Statement as CheckIfFunctionCallExistsVisitor>::visit(&stmt, symbol_table)?;
        }

        Ok(true)
    }
}

#[derive(Debug)]
pub enum Continuation {
    Continue(Option<IdTy>),
    Break(Option<IdTy>),
}

#[derive(Debug)]
pub enum Expr {
    Num(i32),
    Id(String),
    // like addition, multiplication
    Chained(Opcode, Box<Term>, Box<Expr>),
    // not, head, tail, islist
    Unchained(Opcode, Box<Term>),
    // >=, ==
    Dual(Opcode, Box<Term>, Box<Term>),
    Single(Box<Term>),
}

impl CheckIfFunctionCallExistsVisitor for Expr {
    fn visit(&self, symbol_table: &SymbolTable) -> Result<bool> {
        match &*self {
            Expr::Chained(_, term, expr) => {
                <Term as CheckIfFunctionCallExistsVisitor>::visit(&term, symbol_table)?;
                <Expr as CheckIfFunctionCallExistsVisitor>::visit(&expr, symbol_table)?;
            }
            Expr::Unchained(_, term) => {
                <Term as CheckIfFunctionCallExistsVisitor>::visit(&term, symbol_table)?;
            }
            Expr::Dual(_, term1, term2) => {
                <Term as CheckIfFunctionCallExistsVisitor>::visit(&term1, symbol_table)?;
                <Term as CheckIfFunctionCallExistsVisitor>::visit(&term2, symbol_table)?;
            }
            Expr::Single(term) => {
                <Term as CheckIfFunctionCallExistsVisitor>::visit(&term, symbol_table)?;
            }
            _ => {}
        }

        Ok(true)
    }
}

#[derive(Debug)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
    Not,
    Head,
    Tail,
    IsList,
    Or,
    Dot,
    Geq,
    Cmp,
}

#[derive(Debug)]
pub enum Term {
    Num(i64),
    Id(IdTy),
    Call(IdTy, Vec<Box<Expr>>),
}

impl CheckIfFunctionCallExistsVisitor for Term {
    fn visit(&self, symbol_table: &SymbolTable) -> Result<bool> {
        match &*self {
            Term::Call(id, exprs) => {
                if !symbol_table.lookup_symbol(&id) {
                    bail!("Function {} is not defined", id);
                }

                for expr in exprs {
                    expr.visit(symbol_table)?;
                }
            }
            _ => {}
        }

        Ok(true)
    }
}
