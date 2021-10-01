use crate::visitors::*;
use crate::ast::*;
use either::Either;

pub struct PrintVisitor;
pub struct NormalTraversal;

impl Visitor for PrintVisitor{
    fn visit_program<'ctx>(&mut self, program: &'ctx Program) -> Result<()> {
        println!("Visiting program");
        Ok(())
    }

    fn visit_func<'ctx>(&mut self, func: &'ctx Func) -> Result<()> {
        println!("Visiting func");
        Ok(())
    }

    fn visit_statement<'ctx>(&mut self, stmt: &'ctx Statement) -> Result<()> {
        println!("Visiting statement");
        Ok(())
    }
    
    //fn visit_guard(&mut self, label: &Option<IdTy>, guard: &mut Guard) -> Result<()>;

    fn visit_guard<'ctx>(&mut self, guard: &'ctx Guard) -> Result<()> {
        println!("Visiting guard");
        Ok(())
    }

    fn visit_expr<'ctx>(&mut self, expr: &'ctx Expr) -> Result<()> {
        println!("Visiting expr");
        Ok(())
    }

    fn visit_term<'ctx>(&mut self, term: &'ctx Term) -> Result<()> {
        println!("Visiting term");
        Ok(())
    }

    fn visit_struct<'ctx>(&mut self, stru: &'ctx Struct) -> Result<()> {
        println!("Visiting struct");
        Ok(())
    }
}

impl Traversal for NormalTraversal {
    fn traverse(&mut self, visitor: &mut Box<dyn Visitor>, program: &mut Program) -> Result<()> {
           visitor.visit_program(program)?;

            for struc in program.structs.iter() {
                visitor.visit_struct(struc)?;
            }
            
            for function in program.functions.iter() {
                visitor.visit_func(function)?;

                for statement in function.statements.iter() {

                    let expr_or_guard = statement.get_inner();

                    if let Some(expr_or_guard) = expr_or_guard {
                        match expr_or_guard {
                            Either::Left(expr) => {
                                visitor.visit_expr(expr)?;
                            }
                            Either::Right(guards) => {
                                for guard in guards {
                                    visitor.visit_guard(guard)?;
                                } 
                            }
                        }
                    }

                    visitor.visit_statement(statement.as_ref())?;
                    
                }
            }

        Ok(())
    }
}