use std::fs;

mod splitter;
mod parser;
mod codegen;

use clap::Parser;
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

    let compiler = codegen::Compiler::new("main_module");

    
    if parsed_main_file.is_some() {
        compiler.compile(&parsed_main_file.unwrap());
    } else {
        panic!("Could not parse file")
    }

    splitter.print_errors();

    println!("{}", "finished");
}
