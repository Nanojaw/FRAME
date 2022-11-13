use std::fs;

mod splitter;

use clap::Parser;
#[derive(Parser)]
pub struct Cli {
    pub path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let file = fs::read_to_string(&args.path).expect("Could not read file");
    println!("{}", file);

    let block = splitter::split_file(
        file.as_str(),
        args.path.file_name().unwrap().to_str().unwrap(),
    );

    block.unwrap().print(0);

    println!("{}", "finished");
}
