use std::fs;

mod splitter;
mod parser;

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

    // Print errors
    splitter.print_errors();

    println!("{}", "finished");
}
