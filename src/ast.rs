use std::collections::HashSet;
use crate::symbol_table::SymbolTable;
use crate::visitors::CheckIfFunctionCallExistsVisitor;

pub type IdTy = String;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SymbolAlreadyDefined(IdTy),
    SymbolNotDefined(IdTy),
    FunctionNotDefined(IdTy)
}

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
}

impl CheckIfFunctionCallExistsVisitor for Program {
    fn visit(&self, symbol_table: &SymbolTable) -> Result<bool> {
        for function in &self.functions {
            function.visit(symbol_table)?;
        }

        Ok(true)
    }
}

#[derive(Debug)]
pub struct Func {
    pub id: IdTy,
    pub pars: Vec<IdTy>,
    pub statements: Vec<Box<Statement>>,
    /// Symbol table for variables
    symbol_table: SymbolTable,
}

impl Func {
    pub fn new(id: IdTy, pars: Vec<IdTy>, statements: Vec<Box<Statement>>) -> Result<Self> {
        let mut symbol_table = SymbolTable::default();

        for stmt in statements.iter() {
            match &**stmt {
                Statement::Assign(id, _) => {
                    symbol_table.insert(id)?
                },
                Statement::ReAssign(id, _) => {
                    if !symbol_table.lookup_symbol(&id) {
                        return Err(Error::SymbolNotDefined(id.to_string()));
                    }
                },
                _ => {}
            }
        }

        Ok(Self {
            id,
            pars,
            statements,
            symbol_table 
        })
    }
}

impl CheckIfFunctionCallExistsVisitor for Func {
    fn visit(&self, symbol_table: &SymbolTable) -> Result<bool> {
        for stmt in &self.statements {
            stmt.visit(symbol_table)?;
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
            Statement::Ret(expr) => { expr.visit(symbol_table)?; },
            Statement::Assign(_, expr) => { expr.visit(symbol_table)?; },
            Statement::ReAssign(_, expr) => { expr.visit(symbol_table)?; },
            Statement::Conditional(_, guards) => { 
                for guard in guards {
                    guard.visit(symbol_table)?;
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
            expr.visit(symbol_table)?;
        }

        for stmt in &self.statements {
            stmt.visit(symbol_table)?;
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
                term.visit(symbol_table)?;
                expr.visit(symbol_table)?;
            },
            Expr::Unchained(_, term) => {
                term.visit(symbol_table)?;
            },
            Expr::Dual(_, term1, term2) => {
                term1.visit(symbol_table)?;
                term2.visit(symbol_table)?;
            },
            Expr::Single(term) => {
                term.visit(symbol_table)?;
            },
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
    Num(i32),
    Id(IdTy),
    Call(IdTy, Vec<Box<Expr>>),
}

impl CheckIfFunctionCallExistsVisitor for Term {
    fn visit(&self, symbol_table: &SymbolTable) -> Result<bool> {

        match &*self {
            Term::Call(id, exprs) => {
                
                if !symbol_table.lookup_symbol(&id) {
                    return Err(Error::FunctionNotDefined(id.to_string()));
                }

                for expr in exprs {
                    expr.visit(symbol_table)?;
                }
            },
            _ => {}
        }

        Ok(true)
    }
}