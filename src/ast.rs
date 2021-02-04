use std::collections::HashSet;

type IdTy = String;

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Func>,
}

impl Program {
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

#[derive(Debug)]
pub struct Func {
    pub id: IdTy,
    pub pars: Vec<IdTy>,
    pub statements: Vec<Box<Statement>>,
}

#[derive(Debug)]
pub enum Statement {
    Ret(Box<Expr>),
    Assign(IdTy, Box<Expr>),
    ReAssign(IdTy, Box<Expr>),
    Conditional(Option<IdTy>, Vec<Box<Guard>>),
}

#[derive(Debug)]
pub struct Guard {
    pub guard: Option<Box<Expr>>,
    pub statements: Vec<Box<Statement>>,
    pub continuation: Continuation, 
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

