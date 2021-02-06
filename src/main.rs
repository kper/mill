#[macro_use]
extern crate lalrpop_util;

use std::env;
use std::fs::File;
use std::io::Read;
use crate::visitors::CheckIfFunctionCallExistsVisitor;

mod ast;
mod symbol_table;
mod visitors;
mod codegen;

#[cfg(test)]
mod tests;

lalrpop_mod!(pub grammar);

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let mut content = String::new();

    for file in arguments.into_iter().skip(1) {
        let mut file_content = String::new();
        let mut fs = File::open(file).expect("Cannot find file");

        fs.read_to_string(&mut file_content)
            .expect("Cannot read into string");
        content.push_str(&file_content);
    }

    //println!("{}", content);

    let mut ast = grammar::ProgramParser::new().parse(&content).unwrap();

    println!("{:#?}", ast);

    if ast.check_duplicated_names() {
        panic!("Function defined multiple times");
    }

    let functions = ast.get_function_names().unwrap();

    if let Err(err) = ast.visit(&functions) {
        panic!("{:?}", err);
    }

    let codegen = ast.codegen_to_file("main.bc");
    if let Err(err) = codegen {
        panic!("{:?}", err);
    }
}