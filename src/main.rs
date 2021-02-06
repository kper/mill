#[macro_use]
extern crate lalrpop_util;

use crate::visitors::CheckIfFunctionCallExistsVisitor;
use std::env;
use std::fs::File;
use std::io::Read;

mod ast;
mod codegen;
mod symbol_table;
mod visitors;

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

    if let Err(err) = ast.visit(&functions) {
        panic!("{:?}", err);
    }

    info!("=> Finished codegen");
    info!("=> Starting writing file");

    let codegen = ast.codegen_to_file("main.bc");
    if let Err(err) = codegen {
        panic!("{:?}", err);
    }

    info!("=> Finished");
}
