use std::fs;

mod splitter;
//mod parser;

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
    let main_block = splitter.split_file();

    main_block.print(0);

    splitter.print_errors();


    //let parsed_block = block.expect("File was invalid").parse();

    println!("{}", "finished");
}
