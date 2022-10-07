#![allow(unused)]

use clap::Parser;
use gru::Config;
use std::{env, process, error::Error, fs};
use serde::Deserialize;

/// Config file manager

pub struct Repository {
    pub path: String
}

pub struct State {
    pub command: String,
    pub argument: String
}

pub trait Command {

}

pub struct File<'a> {
    local_path: &'a str,
    repo_path: &'a str
}

#[derive(Deserialize, Debug)]
pub struct Configuration<'a> {
    url: &'a str
}

impl<'a> Configuration<'a> {
    pub fn new(path: &'a str) -> Result<Configuration, &'static str> {
        let deserialized = serde_json::from_str(path).unwrap();
        Ok(deserialized)
    }
}

fn main() {
    let content = read_config("gru-settings.json").unwrap_or_else(|err| {
        eprintln!("Problem reading configuration file: {err}");
        process::exit(1);
    });;
    let config = Configuration::new(&content).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {err}");
        process::exit(1);
    });
    println!("Repository url: {}", config.url);
    // let config = Config::new(env::args()).unwrap_or_else(|err| {
    //     eprintln!("Problem passing arguments: {err}");
    //     process::exit(1);
    // });
    
    // if let Err(e) = gru::run(config) {
    //     eprintln!("Application error: {e}");
    //     process::exit(1);
    // }
}

fn read_config<'a>(path: &'a str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string("gru-settings.json")?;
    Ok(contents)
}