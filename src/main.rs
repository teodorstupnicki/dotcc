#![allow(unused)]

use clap::Parser;
use gru::Config;
use std::{env, process, error::Error};

/// Search for a pattern in a file and display the lines that contain it.

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {err}");
        process::exit(1);
    });
    
    if let Err(e) = gru::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}