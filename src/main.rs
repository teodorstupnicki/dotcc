#![allow(unused)]

use clap::Parser;
use std::env;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf
}

fn main() {
    // let pattern = std::env::args().nth(1).expect("No pattern given");
    // let path = std::env::args().nth(2).expect("No path given");
    
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .expect("Could not read file");
    
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
