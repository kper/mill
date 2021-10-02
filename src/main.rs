#[macro_use]
extern crate lalrpop_util;

use std::env;
use std::fs::File;
use std::io::Read;

mod ast;
mod codegen;
mod symbol_table;
mod visitors;
mod traversal;
mod pass;
mod runner;
mod nodes;

use visitors::*;
use pass::Pass;
use runner::Runner;

use crate::traversal::NormalTraversal;
use crate::traversal::CodegenTraversal;

use log::info;

use inkwell::context::Context as LLVM_Context;

use crate::codegen::Codegen;

use anyhow::{Result, Context};

#[cfg(test)]
mod tests;

lalrpop_mod!(pub grammar);

fn main() {
    env_logger::init();

    let arguments: Vec<String> = env::args().collect();

    let mut content = String::new();

    info!("=> Running compiler with {:?}", arguments);

    for file in arguments.into_iter().skip(1) {
        let mut file_content = String::new();
        let mut fs = File::open(file).expect("Cannot find file");

        fs.read_to_string(&mut file_content)
            .expect("Cannot read into string");
        content.push_str(&file_content);
    }

    let mut ast = grammar::ProgramParser::new().parse(&content).unwrap();

    info!("=> Program parsed");

    info!("=> Starting codegen");

    let result = run(ast);

    if let Err(err) = result {
        eprintln!("ERROR: {}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| eprintln!("because: {}", cause));
        std::process::exit(1);
    }
}

fn run(mut ast: ast::Program) -> Result<()> {
    let mut passes = default_passes();
    let context = LLVM_Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();
    let mut runner = Runner;

    runner.run_visitors(&mut passes, &mut ast).context("Running visitors failed")?;
    
    info!("=> Finished visitors");
    let mut visitor = CodegenVisitor::new(module, builder);
    let x = runner.run_codegen(visitor, CodegenTraversal, &mut ast).context("Running codegen failed")?;
    info!("=> Finished codegen");
    info!("=> Starting writing file");

    x.write_bitcode("main.bc")?;

    info!("=> Finished");

    Ok(())
}

pub(crate) fn default_passes() -> Vec<Pass> {
    vec![
        //Pass::new(Box::new(PrintVisitor), Box::new(NormalTraversal)), 
        Pass::new(Box::new(CheckIfFunctionCallExistsVisitor::default()), Box::new(NormalTraversal))
    ]
}