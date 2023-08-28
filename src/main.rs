#![allow(unused)]
use clap::{Parser, arg, Subcommand, Args};
use gru::Configuration;
use std::{env, process, error::Error, fs};
use serde::Deserialize;

/// Config file manager
static CONFIG_FILE_NAME: &str = "gru-settings.json";

#[derive(Debug, Parser)]
struct GruArgs {
    /// Manage configuration files 
   #[clap(subcommand)]
   action: GruCommand,
}

#[derive(Debug, Subcommand)]
pub enum GruCommand {
    /// Check differences between files
    Check(CheckSubcommand),
    /// Install files in filesystem
    Install(InstallSubcommand),
    /// Apply changes to repository
    Apply(ApplySubcommand),
}

#[derive(Debug, Args)]
pub struct CheckSubcommand {
    /// Configuration file path 
    pub file: String,
}

#[derive(Debug, Args)]
pub struct InstallSubcommand {
    /// Configuration file path 
    pub file: String,
}

#[derive(Debug, Args)]
pub struct ApplySubcommand {
    /// Configuration file path 
    pub file: String,
}

pub struct File<'a> {
    local_path: &'a str,
    repo_path: &'a str
}

fn main() {
    let command = GruArgs::parse();
    match command.action {
        GruCommand::Check(subcommand) => run_check(subcommand),
        GruCommand::Install(subcommand) => run_install(subcommand),
        GruCommand::Apply(subcommand) => run_apply(subcommand),
    }
    let content = gru::read_config(CONFIG_FILE_NAME).unwrap_or_else(|err| {
        eprintln!("Problem reading configuration file: {err}");
        process::exit(1);
    });
    let config = Configuration::new(&content).unwrap_or_else(|err| {
        eprintln!("Problem with creating configuration object: {err}");
        process::exit(1);
    });

    println!("Repository url: {}", config.url);
}

fn run_check(subcommand: CheckSubcommand) {

}

fn run_install(subcommand: InstallSubcommand) {

}

fn run_apply(subcommand: ApplySubcommand) {

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        const DEFAULT_CONFIGFILE_NAME: &str = "gru-settings.json";
        let x = gru::read_config(DEFAULT_CONFIGFILE_NAME);
        let check = if let Ok(result) = x {
            true
        } else {
            false
        };
        assert!(check);
    }
}