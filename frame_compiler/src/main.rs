use std::fs;

mod splitter;
mod parser;
mod FrameReturnType;

use clap::Parser;
#[derive(Parser)]
pub struct Cli {
    pub path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let file = fs::read_to_string(&args.path).expect("Could not read file");
    let block = splitter::split_file(file.as_str());

    let parsed_block = block.expect("File was invalid").parse();

    println!("{}", "finished");
}
