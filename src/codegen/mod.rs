use crate::ast::*;
use crate::symbol_table::{LLVMFunctionTable, LLVMSymbolTable};
use crate::visitors::CodegenVisitor;
use std::borrow::Cow;
use std::path::Path;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::{InitializationConfig, Target};
use inkwell::values::{BasicValue, BasicValueEnum};
use inkwell::IntPredicate;

type Result<T> = std::result::Result<T, Error>;

pub struct Codegen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    //execution_engine: ExecutionEngine<'ctx>,
    symbol_table: LLVMSymbolTable<'ctx>,
    function_table: LLVMFunctionTable<'ctx>,
}

impl<'ctx> Codegen<'ctx> {
    pub fn new(context: &'ctx Context, module: Module<'ctx>) -> Codegen<'ctx> {
        Target::initialize_native(&InitializationConfig::default())
            .expect("Failed to initialize native target");

        //let execution_engine = module.create_execution_engine().unwrap();

        Codegen {
            context: context,
            module,
            builder: context.create_builder(),
            //execution_engine,
            symbol_table: LLVMSymbolTable::default(),
            function_table: LLVMFunctionTable::default(),
        }
    }

    pub fn write_bitcode(&self, name: &str) -> Result<()> {
        let path = Path::new(name);

        self.module.write_bitcode_to_path(path);

        Ok(())
    }

    pub fn get_ir(&self) -> String {
        self.module.print_to_string().to_string()
    }
}

impl<'ctx> CodegenVisitor<'ctx> for Codegen<'ctx> {
    fn visit_program(&mut self, program: &'ctx mut Program) -> Result<()> {
        for func in program.functions.iter_mut() {
            let context = &self.context;
            let module = &self.module;

            let i64_type = context.i64_type();

            let func_types = vec![i64_type.into(); func.pars.len()];
            let fn_type = i64_type.fn_type(&func_types, false);
            let function = module.add_function(&func.id, fn_type, None);

            self.function_table.insert(&func.id, function)?;
        }

        for func in program.functions.iter_mut() {
            self.visit_func(func)?;
        }

        Ok(())
    }

    fn visit_func(&mut self, func: &'ctx mut Func) -> Result<()> {
        let context = &self.context;
        let builder = &self.builder;

        let func_ref = self.function_table.get(&func.id).unwrap();

        let basic_block = context.append_basic_block(*func_ref, &func.id);

        builder.position_at_end(basic_block);

        for (i, param) in func.pars.iter().enumerate() {
            let value = func_ref.get_nth_param(i as u32).unwrap();
            let i64_type = self.context.i64_type();
            let ptr = self.builder.build_alloca(i64_type, param);

            let _instr = self.builder.build_store(ptr, value);

            self.symbol_table
                .insert(param, BasicValueEnum::PointerValue(ptr))?;
        }

        for stmt in func.statements.iter() {
            self.visit_statement(stmt)?;
        }

        self.symbol_table.clear();

        Ok(())
    }

    fn visit_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::Ret(expr) => {
                let res = self.visit_expr(expr).map(|x| x.into_owned());
                let ret: Option<&dyn BasicValue> = res.as_ref().map(|x| x as &dyn BasicValue);

                self.builder.build_return(ret);
            }
            Statement::Assign(id, expr) => {
                let res = self.visit_expr(expr).map(|x| x.into_owned());

                if let Some(val) = res {
                    let i64_type = self.context.i64_type();
                    let ptr = self.builder.build_alloca(i64_type, id);

                    let _instr = self.builder.build_store(ptr, val);
                    self.symbol_table
                        .insert(id, BasicValueEnum::PointerValue(ptr))?;
                } else {
                    panic!("No value found");
                }
            }
            Statement::ReAssign(id, expr) => {
                let res = self.visit_expr(expr).map(|x| x.into_owned());
                let ptr = self.symbol_table.get(id);

                if let (Some(val), Some(ptr)) = (res, ptr) {
                    let _instr = self.builder.build_store(ptr.into_pointer_value(), val);
                } else {
                    panic!("No value or ptr found");
                }
            }
            _ => unimplemented!(),
        }

        Ok(())
    }

    fn visit_expr(&mut self, expr: &Expr) -> Option<Cow<BasicValueEnum<'ctx>>> {
        match expr {
            Expr::Num(num) => {
                let i64_type = self.context.i64_type();
                let obj = BasicValueEnum::IntValue(i64_type.const_int(*num as u64, false));

                return Some(Cow::Owned(obj));
            }
            Expr::Id(id) => {
                let var = self.symbol_table.get(id).map(|x| Cow::Borrowed(x));
                if let Some(var) = var {
                    let ptr = var.into_pointer_value();
                    return Some(Cow::Owned(self.builder.build_load(ptr, id)));
                } else {
                    panic!("No entry in symbol table");
                }
            }
            Expr::Single(term) => {
                return self.visit_term(term);
            }
            Expr::Unchained(Opcode::Not, term) => {
                let res = self.visit_term(term).map(|x| x.into_owned());

                if let Some(val) = res {
                    let neg = self
                        .builder
                        .build_not(val.into_int_value(), &self.symbol_table.get_new_name());

                    return Some(Cow::Owned(BasicValueEnum::IntValue(neg)));
                } else {
                    panic!("no value found");
                }
            }
            Expr::Unchained(_, _term) => {
                panic!("Opcode not supported");
            }
            Expr::Dual(Opcode::Cmp, lhs, rhs) => {
                let lhs = self.visit_term(lhs).map(|x| x.into_owned());
                let rhs = self.visit_term(rhs).map(|x| x.into_owned());

                if let (Some(lhs), Some(rhs)) = (lhs, rhs) {
                    let eq = self.builder.build_int_compare(
                        IntPredicate::EQ,
                        lhs.into_int_value(),
                        rhs.into_int_value(),
                        &self.symbol_table.get_new_name(),
                    );

                    return Some(Cow::Owned(BasicValueEnum::IntValue(eq)));
                } else {
                    panic!("No value found");
                }
            }
            Expr::Dual(Opcode::Geq, lhs, rhs) => {
                let lhs = self.visit_term(lhs).map(|x| x.into_owned());
                let rhs = self.visit_term(rhs).map(|x| x.into_owned());

                if let (Some(lhs), Some(rhs)) = (lhs, rhs) {
                    let eq = self.builder.build_int_compare(
                        IntPredicate::SGE,
                        lhs.into_int_value(),
                        rhs.into_int_value(),
                        &self.symbol_table.get_new_name(),
                    );

                    return Some(Cow::Owned(BasicValueEnum::IntValue(eq)));
                } else {
                    panic!("No value found");
                }
            }
            Expr::Chained(op, lhs, rhs) => {
                let lhs = self.visit_term(lhs).map(|x| x.into_owned());
                let rhs = self.visit_expr(rhs).map(|x| x.into_owned());

                if let (Some(lhs), Some(rhs)) = (lhs, rhs) {
                    let res = match op {
                        Opcode::Add => self.builder.build_int_add(
                            lhs.into_int_value(),
                            rhs.into_int_value(),
                            &self.symbol_table.get_new_name(),
                        ),
                        Opcode::Mul => self.builder.build_int_mul(
                            lhs.into_int_value(),
                            rhs.into_int_value(),
                            &self.symbol_table.get_new_name(),
                        ),
                        Opcode::Sub => self.builder.build_int_sub(
                            lhs.into_int_value(),
                            rhs.into_int_value(),
                            &self.symbol_table.get_new_name(),
                        ),
                        _ => panic!("opcode not supported"),
                    };

                    return Some(Cow::Owned(BasicValueEnum::IntValue(res)));
                } else {
                    panic!("No value found");
                }
            }
            _ => return None,
        }
    }

    fn visit_term(&mut self, term: &Term) -> Option<Cow<BasicValueEnum<'ctx>>> {
        match term {
            Term::Num(num) => {
                let i64_type = self.context.i64_type();
                let obj = BasicValueEnum::IntValue(i64_type.const_int(*num as u64, false));

                return Some(Cow::Owned(obj));
            }
            Term::Id(id) => {
                let var = self.symbol_table.get(id).map(|x| Cow::Borrowed(x));
                if let Some(var) = var {
                    let ptr = var.into_pointer_value();
                    return Some(Cow::Owned(self.builder.build_load(ptr, id)));
                } else {
                    panic!("No entry in symbol table");
                }
            }
            Term::Call(id, pars) => {
                let arguments: Vec<_> = pars
                    .iter()
                    .map(|x| self.visit_expr(x).map(|x| x.into_owned()))
                    .map(|x| x.unwrap())
                    .collect();

                if let Some(func_ref) = self.function_table.get(id) {
                    return Some(Cow::Owned(
                        self.builder
                            .build_call(*func_ref, &arguments, id)
                            .try_as_basic_value()
                            .left()
                            .unwrap(),
                    ));
                } else {
                    panic!("Function not found");
                }
            }
        }
    }
}
