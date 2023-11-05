use crate::symbol_table::{FunctionSignature, SymbolTable};
use anyhow::{bail, Result};
use std::fmt;
use std::hash::{Hash, Hasher};

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
}

#[derive(Debug, Clone)]
pub struct Func {
    /// Name of the function
    pub id: Identifier,
    /// Parameters of the function
    pub pars: Vec<Identifier>,
    /// The statements of the function
    pub statements: Vec<Box<Statement>>,
    /// Return type of the function. This is None when void.
    pub ret_ty: Option<DataType>,
}

impl Func {
    pub fn new(
        id: Identifier,
        pars: Vec<Identifier>,
        statements: Vec<Box<Statement>>,
        ret_ty: Option<DataType>,
    ) -> Result<Self> {
        let mut symbol_table = SymbolTable::default();

        for stmt in statements.iter() {
            match &**stmt {
                Statement::Assign(id, _) => {
                    if !symbol_table.lookup_symbol(&id.get_name()) {
                        bail!("Symbol {} is not defined", id);
                    }
                }
                Statement::Definition(id, _) => symbol_table.insert(id.get_name())?,
                _ => {}
            }
        }

        Ok(Self {
            id,
            pars,
            statements,
            ret_ty,
        })
    }

    pub fn get_signature(&self) -> FunctionSignature {
        let arguments_ty = self
            .pars
            .iter()
            .map(|x| x.ty.clone().expect("Argument must have type"))
            .collect();
        let ret_ty = self.ret_ty.clone();

        FunctionSignature::new(arguments_ty, ret_ty)
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    RetVoid,
    Ret(Box<Expr>),
    Assign(Identifier, Box<Expr>),
    Definition(Identifier, Box<Expr>),
    Conditional(Box<Expr>, Vec<Box<Statement>>),
}

impl Statement {
    pub fn get_inner(&self) -> Option<&Box<Expr>> {
        return match self {
            Statement::Ret(expr) => Some(expr),
            Statement::Assign(_, expr) => Some(expr),
            Statement::Definition(_, _) => None,
            Statement::RetVoid => None,
            Statement::Conditional(expr, _) => Some(expr),
        };
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Struct(Identifier),
    Binary(Opcode, Box<Term>, Box<Term>),
    Term(Box<Term>),
    Call(Identifier, Vec<Box<Expr>>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
    Not,
    Head,
    Tail,
    Or,
    Geq,
    Cmp,
}

#[derive(Debug, Clone)]
pub enum Term {
    Num(i64),
    Id(Identifier),
}
