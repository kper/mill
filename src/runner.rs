use crate::pass::Pass;
use crate::ast::*;
use anyhow::Result;

pub struct Runner;

impl Runner {
    /**
     * Run the visitors
     */
    pub fn run_visitors(&mut self, passes: Vec<Pass>, program: &mut Program) -> Result<()> {

        for mut pass in passes.into_iter() {
            pass.run(program)?;
        }

        
        Ok(())
    }
}