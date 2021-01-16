type IdTy = String;

pub struct Func {
    pub id: IdTy,
    pub pars: Vec<IdTy>,
    pub statements: Vec<Box<Statement>>,
}

pub enum Statement {
    Ret(Box<Expr>),
    Assign(IdTy, Box<Expr>),
    ReAssign(IdTy, Box<Expr>),
}

pub enum Expr {
    Num(i32),
    Id(String),
    Op(Opcode, Box<Term>),
    Single(Box<Term>),
} 

pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
    Not,
    Head,
    Tail,
    IsList,
}

pub enum Term {
    Num(i32),
    Id(IdTy),
}

