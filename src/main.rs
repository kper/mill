#[macro_use]
extern crate lalrpop_util;
extern crate core;

use std::fs::File;
use std::io::Read;

mod ast;
mod codegen;
mod nodes;
mod pass;
mod runner;
mod symbol_table;
mod traversal;
mod utils;
mod visitors;
mod lir;

use runner::Runner;
use clap::Parser;

use llvm_sys::bit_writer::LLVMWriteBitcodeToFile;
use llvm_sys::core::*;

use log::info;

use anyhow::{Context, Result};
use llvm_sys::analysis::{LLVMVerifierFailureAction, LLVMVerifyModule};

use crate::lir::lowering::LoweringPass;

#[macro_export]
macro_rules! c_str {
    ($s:expr) => {
        format!("{}\0", $s).as_ptr() as *const i8
    };
}

#[cfg(test)]
mod tests;

lalrpop_mod!(pub grammar);

/// A simple compiler
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    print_lowering: bool,
    #[arg(short, long)]
    files: Vec<String>,
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    info!("=> Running compiler with {:?}", args);

    let mut content = String::new();
    for file in args.files.into_iter().skip(1) {
        let mut file_content = String::new();
        let mut fs = File::open(file).expect("Cannot find file");

        fs.read_to_string(&mut file_content)
            .expect("Cannot read into string");
        content.push_str(&file_content);
    }

    let ast = grammar::ProgramParser::new().parse(&content).unwrap();

    info!("=> Program parsed");

    info!("=> Staring lowering");
    let mut lowering_pass = LoweringPass;
    let lowered = lowering_pass.lower(&ast).unwrap();

    if args.print_lowering {
        use graphviz_rust::printer::{DotPrinter, PrinterContext};

        let graph = lowered.print();
        println!("{}", graph.print(&mut PrinterContext::default()));
    }

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
    let mut runner = Runner;

    unsafe {
        let context = LLVMContextCreate();
        let module = LLVMModuleCreateWithName(c_str!("main"));
        let builder = LLVMCreateBuilderInContext(context);

        //let mut codegen = Codegen::new(context, module, builder);

        /*
        runner
            .run_visitors(&mut passes, &mut ast)
            .context("Running visitors failed")?;
        */
        LLVMVerifyModule(
            module,
            LLVMVerifierFailureAction::LLVMAbortProcessAction,
            std::ptr::null_mut(),
        );

        info!("=> Starting writing file");

        LLVMWriteBitcodeToFile(module, c_str!("main.bc"));

        LLVMDisposeBuilder(builder);
        LLVMDisposeModule(module);
        LLVMContextDispose(context);

        info!("=> Finished");
    }

    Ok(())
}