use crate::ast::*;
use crate::symbol_table::*;
use crate::visitors::CodegenVisitor;
use std::borrow::Cow;
use std::path::Path;

use anyhow::{bail, Context, Result};
use inkwell::builder::Builder;
use inkwell::context::Context as LLVM_Context;
use inkwell::module::Module;
use inkwell::targets::{InitializationConfig, Target};
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValue, BasicValueEnum};
use inkwell::IntPredicate;
use log::debug;

pub struct Codegen<'ctx> {
    pub context: &'ctx LLVM_Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    //execution_engine: ExecutionEngine<'ctx>,
    symbol_table: LLVMSymbolTable<'ctx>,
    function_table: LLVMFunctionTable<'ctx>,
    block_table: LLVMBlockTable<'ctx>,
    struct_table: LLVMStructTable<'ctx>,
}

impl<'ctx> Codegen<'ctx> {
    pub fn new(context: &'ctx LLVM_Context, module: Module<'ctx>, builder: Builder<'ctx>) -> Codegen<'ctx> {
        Target::initialize_native(&InitializationConfig::default())
            .expect("Failed to initialize native target");

        
        //let execution_engine = module.create_execution_engine().unwrap();

        Codegen {
            context,
            module,
            builder,
            //execution_engine,
            symbol_table: LLVMSymbolTable::default(),
            function_table: LLVMFunctionTable::default(),
            block_table: LLVMBlockTable::default(),
            struct_table: LLVMStructTable::default(),
        }
    }

    /* 
    pub fn get_context(&self) -> &LLVM_Context {
        &self.context
    }*/

    pub fn get_module(&self) -> &Module<'ctx> {
        &self.module
    }

    pub fn get_mut_module(&mut self) -> &mut Module<'ctx> {
        &mut self.module
    }

    pub fn get_builder(&self) -> &Builder<'ctx> {
        &self.builder
    }

    pub fn get_mut_builder(&mut self) -> &mut Builder<'ctx> {
        &mut self.builder
    }

    pub fn get_function_table(&self) -> &LLVMFunctionTable<'ctx> {
        &self.function_table
    }

    pub fn get_mut_function_table(&mut self) -> &mut LLVMFunctionTable<'ctx> {
        &mut self.function_table
    }

    pub fn get_block_table(&self) -> &LLVMBlockTable<'ctx> {
        &self.block_table
    }

    pub fn get_symtable(&self) -> &LLVMSymbolTable<'ctx> {
        &self.symbol_table
    }

    pub fn get_mut_symtable(&mut self) -> &mut LLVMSymbolTable<'ctx> {
        &mut self.symbol_table
    }
    

    pub fn write_bitcode(&self, name: &str) -> Result<()> {
        let path = Path::new(name);

        self.module.write_bitcode_to_path(path);

        Ok(())
    }

    pub fn get_ir(&self) -> String {
        self.module.print_to_string().to_string()
    }

    /*
    pub fn get_llvm_type(&self, ty: &DataType) -> Result<BasicTypeEnum<'ctx>> {
        match ty {
            DataType::Int => {
                return Ok(BasicTypeEnum::IntType(self.context.i64_type()));
            }
            DataType::Struct(id) => {
                let (_, ty) = self
                    .struct_table
                    .get(id.get_name())
                    .ok_or_else(|| (anyhow!("Cannot find struct")))?;

                return Ok(BasicTypeEnum::StructType(*ty));
            }
        }
    }*/
}

/* 
impl<'ctx> CodegenVisitorOld<'ctx> for Codegen<'ctx> {
    fn visit_program(&mut self, program: &'ctx mut Program) -> Result<()> {
        debug!("Visiting program");

        for func in program.functions.iter_mut() {
            let context = &self.context;
            let module = &self.module;

            let i64_type = context.i64_type();

            let func_types = vec![i64_type.into(); func.pars.len()];
            let fn_type = i64_type.fn_type(&func_types, false);
            let function = module.add_function(&func.id.get_name(), fn_type, None);

            self.function_table.insert(&func.id.get_name(), function)?;

            debug!(
                "=> Saved function {}({:?}) into the function table",
                func.id, func_types
            );
        }
        // TODO check for structs
        // TODO check fields for structs

        for mystruct in program.structs.iter_mut() {
            self.visit_struct(mystruct)?;
        }

        for func in program.functions.iter_mut() {
            self.visit_func(func)?;
        }

        Ok(())
    }

    fn visit_func(&mut self, func: &'ctx mut Func) -> Result<()> {
        debug!("Visiting func {}({:?})", func.id, func.pars);
        let context = &self.context;
        let builder = &self.builder;

        let func_ref = self.function_table.get(func.id.get_name()).unwrap();

        let basic_block = context.append_basic_block(*func_ref, func.id.get_name());

        builder.position_at_end(basic_block);

        for (i, param) in func.pars.iter().enumerate() {
            let value = func_ref.get_nth_param(i as u32).unwrap();
            let i64_type = self.context.i64_type();
            let ptr = self.builder.build_alloca(i64_type, param.get_name());

            let _instr = self.builder.build_store(ptr, value);

            self.symbol_table.insert(
                param.get_name(),
                (param.clone(), BasicValueEnum::PointerValue(ptr)),
            )?;
            debug!("Allocating functions parameter {}", param);
        }

        for stmt in func.statements.iter_mut() {
            self.visit_statement(stmt, &func.id)?;
        }

        self.symbol_table.clear();

        Ok(())
    }

    fn visit_statement(&mut self, stmt: &mut Statement, func: &IdTy) -> Result<()> {
        debug!("Visiting statement");

        match stmt {
            Statement::Ret(ref mut expr) => {
                debug!("Statement is a return statement");

                let res = self.visit_expr(expr, &None).map(|x| x.into_owned());
                let ret: Option<&dyn BasicValue> = res.as_ref().map(|x| x as &dyn BasicValue);

                self.builder.build_return(ret);
            }
            Statement::Assign(id, ref mut expr) => {
                debug!("Statement is an assignment");

                let res = self.visit_expr(expr, &id.ty).map(|x| x.into_owned());

                if let Some(val) = res {
                    let i64_type = self.context.i64_type();
                    let ptr = self.builder.build_alloca(i64_type, id.get_name());

                    let _instr = self.builder.build_store(ptr, val);
                    self.symbol_table.insert(
                        id.get_name(),
                        (id.clone(), BasicValueEnum::PointerValue(ptr)),
                    )?;
                } else {
                    panic!("No value found");
                }
            }
            Statement::Allocate(symbol, alloc_struct) => {
                debug!("Statement is an allocation");

                let (_, ty) = self.struct_table.get(alloc_struct.get_name()).unwrap();
                let ptr = self
                    .builder
                    .build_alloca(BasicTypeEnum::StructType(*ty), symbol.get_name());

                debug!("=> the type of {} is {:?}", symbol.get_name(), symbol.ty);

                //let _instr = self.builder.build_store(ptr, val);
                debug!("Allocating struct in variable {}", symbol.get_name());
                self.symbol_table.insert(
                    symbol.get_name(),
                    (symbol.clone(), BasicValueEnum::PointerValue(ptr)),
                )?;
            }
            Statement::ReAssign(id, ref mut expr) => {
                debug!("Statement is a reassignment");

                let res = self.visit_expr(expr, &id.ty).map(|x| x.into_owned());

                if res.is_none() {
                    bail!("Evaluated expression is None");
                }
                if let Some((symbol, ptr)) = self.symbol_table.get_both(id.get_name()) {
                    if !id.is_field_access() {
                        let _instr = self
                            .builder
                            .build_store(ptr.into_pointer_value(), res.unwrap());
                    } else {
                        if let Some(field) = id.get_field() {
                            let name_struct = match &symbol.ty.as_ref().unwrap() {
                                DataType::Struct(id) => id.get_name(),
                                _ => bail!("Field accessed on primary type"),
                            };

                            let (my_struct, _llvm_struct) =
                                self.struct_table.get(name_struct).unwrap();
                            let index = my_struct.get_id_by_field_name(field.get_name()).unwrap();

                            let element_ptr = self
                                .builder
                                .build_struct_gep(
                                    ptr.into_pointer_value(),
                                    index as u32,
                                    field.get_name(),
                                )
                                .unwrap();

                            self.builder
                                .build_store(element_ptr, res.unwrap());

                            return Ok(());
                        } else {
                            bail!(
                                "Symbol {} has fields, but they were not defined.",
                                id.get_name()
                            );
                        }
                    }
                } else {
                    bail!("Symbol {} not found", id);
                }
            }
            Statement::Conditional(id, ref mut guards) => {
                debug!("Statement is a conditional ({:?})", id);

                for guard in guards.iter_mut() {
                    self.visit_guard(id, guard, func)
                        .context("Visiting guard in the conditional")?;
                }
            }
        }

        Ok(())
    }

    fn visit_guard(
        &mut self,
        label: &Option<IdTy>,
        guard: &mut Guard,
        function_id: &IdTy,
    ) -> Result<()> {
        debug!("Visiting guard");

        let cond_block = self.context.append_basic_block(
            *self.function_table.get(function_id.get_name()).unwrap(),
            &self.function_table.get_new_name(),
        );

        self.builder.build_unconditional_branch(cond_block);

        let after_cond_block = self.context.append_basic_block(
            *self.function_table.get(function_id.get_name()).unwrap(),
            &self.function_table.get_new_name(),
        );
        if let Some(label) = label {
            debug!("Saving block with label {}", label);
            self.block_table
                .insert(label.get_name(), (cond_block, after_cond_block))?;
        }

        self.builder.position_at_end(cond_block);

        if let Some(condition) = &mut guard.guard {
            debug!("=> has a condition");

            let res = self
                .visit_expr(&mut *condition, &None)
                .map(|x| x.into_owned());

            let then_block = self.context.append_basic_block(
                *self.function_table.get(function_id.get_name()).unwrap(),
                &self.function_table.get_new_name(),
            );

            let else_block = self.context.append_basic_block(
                *self.function_table.get(function_id.get_name()).unwrap(),
                &self.function_table.get_new_name(),
            );

            if let Some(cond) = res {
                self.builder.build_conditional_branch(
                    cond.into_int_value(),
                    then_block,
                    else_block,
                );
            } else {
                bail!("Conditional expression is None");
            }

            self.builder.position_at_end(then_block);

            let cpy_symbols = self.symbol_table.clone();
            for stmt in guard.statements.iter_mut() {
                self.visit_statement(stmt, function_id)?;
            }
            self.symbol_table = cpy_symbols;

            match guard.continuation {
                Continuation::Break(None) => {
                    self.builder.build_unconditional_branch(else_block);
                }
                Continuation::Continue(None) => {
                    self.builder.build_unconditional_branch(cond_block);
                }
                Continuation::Break(Some(ref label)) => {
                    self.builder.build_unconditional_branch(
                        self.block_table.get(label.get_name()).unwrap().1,
                    );
                }
                Continuation::Continue(Some(ref label)) => {
                    self.builder.build_unconditional_branch(
                        self.block_table.get(label.get_name()).unwrap().0,
                    );
                }
            }

            self.builder.position_at_end(else_block);
        } else {
            debug!("=> has no condition");

            let basic_block = self.context.append_basic_block(
                *self.function_table.get(function_id.get_name()).unwrap(),
                &self.function_table.get_new_name(),
            );

            self.builder.position_at_end(basic_block);

            let cpy_symbols = self.symbol_table.clone();
            for stmt in guard.statements.iter_mut() {
                self.visit_statement(stmt, function_id)?;
            }
            self.symbol_table = cpy_symbols;

            match guard.continuation {
                Continuation::Break(None) => {
                    self.builder.build_unconditional_branch(after_cond_block);
                }
                Continuation::Continue(None) => {
                    self.builder.build_unconditional_branch(cond_block);
                }
                Continuation::Break(Some(ref label)) => {
                    self.builder.build_unconditional_branch(
                        self.block_table.get(label.get_name()).unwrap().1,
                    );
                }
                Continuation::Continue(Some(ref label)) => {
                    self.builder.build_unconditional_branch(
                        self.block_table.get(label.get_name()).unwrap().0,
                    );
                }
            }
        }

        self.builder.position_at_end(after_cond_block);

        Ok(())
    }

    fn visit_expr(
        &mut self,
        expr: &mut Expr,
        _ty: &Option<DataType>,
    ) -> Option<Cow<BasicValueEnum<'ctx>>> {
        debug!("Visiting expr");

        match expr {
            Expr::Num(num) => {
                debug!("=> is a number {}", num);

                let i64_type = self.context.i64_type();
                let obj = BasicValueEnum::IntValue(i64_type.const_int(*num as u64, false));

                return Some(Cow::Owned(obj));
            }
            Expr::Id(id) => {
                debug!("=> is an ident {}", id);

                let var = self
                    .symbol_table
                    .get(id.get_name())
                    .map(|x| Cow::Borrowed(x));
                if let Some(var) = var {
                    let ptr = var.into_pointer_value();
                    return Some(Cow::Owned(self.builder.build_load(ptr, id.get_name())));
                } else {
                    panic!("No entry in symbol table");
                }
            }
            Expr::Single(term) => {
                debug!("=> is term");

                return self.visit_term(term);
            }
            Expr::Unchained(Opcode::Not, term) => {
                debug!("=> is term");

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
                debug!("=> has two operands for Comparison");

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
                debug!("=> has two operands for GEQ");

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
                debug!("=> chained");

                let lhs = self.visit_term(lhs).map(|x| x.into_owned());
                let rhs = self.visit_expr(rhs, &None).map(|x| x.into_owned());

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

    fn visit_term(&mut self, term: &mut Term) -> Option<Cow<BasicValueEnum<'ctx>>> {
        debug!("Visit term");

        match term {
            Term::Num(num) => {
                debug!("=> term is a number {}", num);

                let i64_type = self.context.i64_type();
                let obj = BasicValueEnum::IntValue(i64_type.const_int(*num as u64, false));

                return Some(Cow::Owned(obj));
            }
            Term::Id(id) => {
                debug!("=> term is an ident {}", id);

                let var = self
                    .symbol_table
                    .get(id.get_name())
                    .map(|x| Cow::Borrowed(x));
                if let Some(var) = var {
                    let ptr = var.into_pointer_value();
                    return Some(Cow::Owned(self.builder.build_load(ptr, id.get_name())));
                } else {
                    panic!("No entry in symbol table");
                }
            }
            Term::Object(symbol, field) => {
                debug!("=> term is a field access {}", field);

                let sym_ref = self.symbol_table.get_both(symbol.get_name());

                if let Some((symbol, llvm_sym)) = sym_ref {
                    if let Some(DataType::Struct(ty)) = &symbol.ty {
                        let (my_struct, _llvm_struct) =
                            self.struct_table.get(ty.get_name()).unwrap();
                        let index = my_struct.get_id_by_field_name(field.get_name()).unwrap();

                        let ptr = llvm_sym.into_pointer_value();

                        // Getting the pointer to the field's struct
                        let element_ptr = self
                            .builder
                            .build_struct_gep(ptr, index as u32, field.get_name())
                            .unwrap();

                        // Loading the pointer of the field
                        return Some(Cow::Owned(
                            self.builder.build_load(element_ptr, field.get_name()),
                        ));
                    } else {
                        panic!("Identfier has wrong type");
                    }
                } else {
                    panic!("Symbol not found");
                }
            }
            Term::Call(id, ref mut pars) => {
                debug!("=> term is a call {}({:?})", id, pars);

                let arguments: Vec<_> = pars
                    .iter_mut()
                    .map(|x| self.visit_expr(x, &id.ty).map(|x| x.into_owned()))
                    .map(|x| x.unwrap())
                    .collect();

                //TODO check types

                if let Some(func_ref) = self.function_table.get(id.get_name()) {
                    return Some(Cow::Owned(
                        self.builder
                            .build_call(*func_ref, &arguments, id.get_name())
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

    fn visit_struct(&mut self, mystruct: &Struct) -> Result<()> {
        debug!("Visit struct");

        let i64_ty = self.context.i64_type();

        let mut field_types: Vec<BasicTypeEnum> = Vec::new();

        for (_i, field) in mystruct.fields.iter().enumerate() {
            match &field.ty {
                DataType::Int => {
                    debug!("Struct {} has i64 field", mystruct.name.get_name());
                    field_types.push(BasicTypeEnum::IntType(i64_ty));
                }
                DataType::Struct(ty) => {
                    let (_, struct_ty) = self.struct_table.get(ty.get_name()).unwrap();

                    field_types.push(BasicTypeEnum::StructType(*struct_ty));
                }
            }
        }

        let struct_ty = self.context.struct_type(field_types.as_slice(), false);
        /*let struct_ptr_ty = struct_ty.ptr_type(AddressSpace::Generic);

        for (i, field) in mystruct.fields.iter().enumerate() {
            self.builder.build_struct_gep(struct_ptr_ty, i, field);
        }*/

        //TODO allow recursive datatypes
        self.struct_table
            .insert(mystruct.name.get_name(), (mystruct.clone(), struct_ty))?;

        Ok(())
    }
}
*/