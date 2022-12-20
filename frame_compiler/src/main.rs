use std::{fs::{self, File}, io::Write};

mod splitter;
mod translator;

use clap::Parser;
#[derive(Parser)]
pub struct Cli {
    pub path: std::path::PathBuf,
}

fn main() -> Result<(), String> {
    let args = Cli::parse();
    let file = fs::read_to_string(&args.path).expect("Could not read file");

    // Parse file into a vector of blocks
    let mut splitter = splitter::Splitter::new(&file);

    let yes = splitter::split_file(&mut splitter)?;

    let mut cpp = translator::to_cpp_file(yes, 1, ' ')?;
    cpp = cpp[0..cpp.len() - 1].to_string();

    println!("{}", cpp);

    let mut cpp_file = File::create("frame_program.cpp");

    if cpp_file.is_err() {
        return Err(cpp_file.err().unwrap().to_string());
    }

    let written_file = cpp_file.unwrap().write_all(cpp.as_bytes());

    if written_file.is_err() {
        return Err(written_file.err().unwrap().to_string());
    }

    let yes = std::process::Command::new("clang++").args(["frame_program.cpp", "-O3", "-Wall"]).output().unwrap();
    
    if !yes.status.success() {
        println!("{}", String::from_utf8_lossy(&yes.stderr));
    }
    
    
    
    //cmd_out = std::process::Command::new("./a.out").output().unwrap();

    Ok(())
}
