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

use log::info;

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

    if ast.check_duplicated_names() {
        panic!("Function defined multiple times");
    }

    info!("No duplicated names");

    let functions = ast.get_function_names().unwrap();

    info!("=> Starting codegen");

    let mut runner = Runner;
    if let Err(err) = runner.run_visitors(vec![
        Pass::new(Box::new(PrintVisitor), Box::new(NormalTraversal)), 
        Pass::new(Box::new(CheckIfFunctionCallExistsVisitor::default()), Box::new(NormalTraversal))
    ], &mut ast) {
        eprintln!("ERROR: {}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| eprintln!("because: {}", cause));
        std::process::exit(1);
    }

    info!("=> Finished codegen");
    info!("=> Starting writing file");

    let codegen = ast.codegen_to_file("main.bc");
    if let Err(err) = codegen {
        eprintln!("ERROR: {}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| eprintln!("because: {}", cause));
        std::process::exit(1);
    }

    info!("=> Finished");
}
