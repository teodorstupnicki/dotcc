#![allow(unused)]

use clap::Parser;
use gru::Configuration;
use std::{env, process, error::Error, fs};
use serde::Deserialize;

/// Config file manager
static CONFIG_FILE_NAME: &str = "gru-settings.json";
pub struct Repository {
    pub path: String
}

pub enum Command {
    Check,
    Install,
    Apply
}

pub struct File<'a> {
    local_path: &'a str,
    repo_path: &'a str
}

fn main() {
    let content = gru::read_config(CONFIG_FILE_NAME).unwrap_or_else(|err| {
        eprintln!("Problem reading configuration file: {err}");
        process::exit(1);
    });;
    let config = Configuration::new(&content).unwrap_or_else(|err| {
        eprintln!("Problem with creating configuration object: {err}");
        process::exit(1);
    });

    println!("Repository url: {}", config.url);
}