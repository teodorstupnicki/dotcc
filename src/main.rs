#![allow(unused)]

use clap::Parser;
use std::{env, process, error::Error};

/// Search for a pattern in a file and display the lines that contain it.

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem passing arguments: {err}");
        process::exit(1);
    });
    
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}