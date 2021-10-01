use crate::visitors::*;
use crate::ast::*;
use either::Either;

pub struct PrintVisitor;
pub struct NormalTraversal;

impl<'ctx> Visitor<'ctx> for PrintVisitor{
    fn visit_program(&mut self, program: &'ctx mut Program) -> Result<()> {
        println!("Visiting program");
        Ok(())
    }

    fn visit_func(&mut self, func: &'ctx mut Func) -> Result<()> {
        println!("Visiting func");
        Ok(())
    }

    fn visit_statement(&mut self, stmt: &mut Statement) -> Result<()> {
        println!("Visiting statement");
        Ok(())
    }
    
    //fn visit_guard(&mut self, label: &Option<IdTy>, guard: &mut Guard) -> Result<()>;

    fn visit_guard(&mut self, guard: &mut Guard) -> Result<()> {
        println!("Visiting guard");
        Ok(())
    }

    fn visit_expr(&mut self, expr: &mut Expr) -> Result<()> {
        println!("Visiting expr");
        Ok(())
    }

    fn visit_term(&mut self, term: &mut Term) -> Result<()> {
        println!("Visiting term");
        Ok(())
    }

    fn visit_struct(&mut self, stru: &mut Struct) -> Result<()> {
        println!("Visiting struct");
        Ok(())
    }
}

impl Traversal for NormalTraversal {
    fn traverse<'ctx>(&mut self, visitor: &mut Box<dyn Visitor<'ctx>>, program: &'ctx mut Program) -> Result<()> {
           visitor.visit_program(program)?;

            for struc in program.structs.iter_mut() {
                visitor.visit_struct(struc)?;
            }
            
            for function in program.functions.iter_mut() {
                visitor.visit_func(function)?;

                for statement in function.statements.iter_mut() {

                    let expr_or_guard = statement.get_mut_inner();

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

                    visitor.visit_statement(statement.as_mut())?;
                    
                }
            }

        Ok(())
    }
}