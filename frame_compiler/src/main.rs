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

    // Parse file into a vector of blocks
    let mut splitter = splitter::Splitter::new(&file);

    let yes = splitter::split_file(&mut splitter);

    if yes.is_err() {
        let error = yes.err().unwrap();
        println!("{}", error);
        return;
    }

    println!("{}", "finished");
}
