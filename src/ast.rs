use crate::codegen::Codegen;
use crate::symbol_table::SymbolTable;
use crate::visitors::CodegenVisitor;
use crate::visitors::Visitor;
use either::Either;
use anyhow::{bail, Result};
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};
use crate::traversal::Traversal;

pub type IdTy = Identifier;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Identifier {
    id: String,
    pos_l: usize,
    pos_r: usize,
    pub ty: Option<DataType>,
    field: Option<Box<Identifier>>,
}

impl Identifier {
    pub fn new(id: String, pos_l: usize, pos_r: usize, ty: Option<DataType>) -> Self {
        Self {
            id,
            pos_l,
            pos_r,
            ty,
            field: None,
        }
    }

    pub fn update_ty(mut self, ty: DataType) -> Self {
        self.ty = Some(ty);
        self
    }

    pub fn get_name(&self) -> &String {
        &self.id
    }

    pub fn update_field_access(mut self, field: Option<Identifier>) -> Self {
        self.field = field.map(|x| Box::new(x));
        self
    }

    pub fn is_field_access(&self) -> bool {
        self.field.is_some()
    }

    pub fn get_field(&self) -> &Option<Box<Identifier>> {
        &self.field
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Hash for Identifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Func>,
    pub structs: Vec<Struct>,
}

#[derive(Debug, Clone)]
pub enum Decl {
    Func(Func),
    Struct(Struct),
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Struct {
    pub name: Identifier,
    pub fields: Vec<Field>,
}

impl Struct {
    pub fn new(name: Identifier, fields: Vec<Field>) -> Result<Self> {
        Ok(Self { name, fields })
    }

    /// Given the field name, return the index of
    /// the field in the struct.
    pub fn get_id_by_field_name(&self, field: &String) -> Result<usize> {
        for (i, f) in self.fields.iter().enumerate() {
            if f.get_name() == field {
                return Ok(i);
            }
        }

        bail!("Field {} was not found", field);
    }
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Field {
    name: Identifier,
    pub ty: DataType,
}

impl Field {
    pub fn new(name: Identifier, ty: DataType) -> Self {
        Self { name, ty }
    }

    pub fn get_name(&self) -> &String {
        &self.name.get_name()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum DataType {
    Int,
    Struct(Box<Identifier>),
}

impl Program {
    /// Get all function names
    pub fn get_function_names(&self) -> Result<SymbolTable> {
        let mut set = SymbolTable::default();

        for name in self.functions.iter().map(|w| &w.id) {
            set.insert(name.get_name())?;
        }

        Ok(set)
    }

    /// Check if any functions has a duplicated name
    /// Returns `true` if the name already exists. Otherwise return `false`.
    pub fn check_duplicated_names(&self) -> bool {
        let mut set = HashSet::new();

        for name in self.functions.iter().map(|w| w.id.get_name()) {
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

#[derive(Debug, Clone)]
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
                Statement::Assign(id, _) => symbol_table.insert(id.get_name())?,
                Statement::Allocate(id, _) => symbol_table.insert(id.get_name())?,
                Statement::ReAssign(id, _) => {
                    if !symbol_table.lookup_symbol(&id.get_name()) {
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

#[derive(Debug, Clone)]
pub enum Statement {
    Ret(Box<Expr>),
    Assign(IdTy, Box<Expr>),
    ReAssign(IdTy, Box<Expr>),
    Conditional(Option<IdTy>, Vec<Box<Guard>>),
    Allocate(IdTy, IdTy),
}

impl Statement {
    pub fn get_inner(&self) -> Option<Either<&Box<Expr>, &Vec<Box<Guard>>>> {
        match self {
            Statement::Ret(expr) => {
                return Some(Either::Left(expr));
            }
            Statement::Assign(_, expr) => {
                return Some(Either::Left(expr));
            }
            Statement::ReAssign(_, expr) => {
                return Some(Either::Left(expr));
            }
            Statement::Conditional(_, guards) => {
                return Some(Either::Right(guards));
            }
            Statement::Allocate(_, _) => {
                return None;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Guard {
    pub guard: Option<Box<Expr>>,
    pub statements: Vec<Box<Statement>>,
    pub continuation: Continuation,
}

#[derive(Debug, Clone)]
pub enum Continuation {
    Continue(Option<IdTy>),
    Break(Option<IdTy>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Num(i32),
    Id(Identifier),
    Struct(Identifier),
    // like addition, multiplication
    Chained(Opcode, Box<Term>, Box<Expr>),
    // not, head, tail, islist
    Unchained(Opcode, Box<Term>),
    // >=, ==
    Dual(Opcode, Box<Term>, Box<Term>),
    Single(Box<Term>),
}

#[derive(Debug, Clone)]
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
    Geq,
    Cmp,
}

#[derive(Debug, Clone)]
pub enum Term {
    Num(i64),
    Id(IdTy),
    Object(IdTy, IdTy),
    Call(IdTy, Vec<Box<Expr>>),
}