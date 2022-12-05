use std::{fs, collections::HashMap};

mod splitter;
mod parser;
mod codegen;

use clap::Parser;
use inkwell::context::Context;
#[derive(Parser)]
pub struct Cli {
    pub path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let file = fs::read_to_string(&args.path).expect("Could not read file");
    
    // Create splitter and split file
    let mut splitter = splitter::Splitter::new(&file);
    let split_main_file = splitter.split_file();

    // Parse file
    let parsed_main_file = split_main_file.parse();

    let context = Context::create();
    let module = context.create_module("main_module");
    let mut codegen = codegen::CodeGen {
        context: &context,
        module,
        builder: context.create_builder(),
        variables: HashMap::new()
    };

    codegen.compile(&parsed_main_file.unwrap());

    splitter.print_errors();

    println!("{}", "finished");
}
