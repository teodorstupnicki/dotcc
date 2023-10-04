#![allow(unused)]
use clap::{Parser, arg, Subcommand, Args};
use dotcc::Configuration;
use std::{env, process, error::Error, fs, string, path::PathBuf};
use serde::Deserialize;
use dirs;

/// Config file manager
static CONFIG_FILE_NAME: &str = "dotcc-config.json";

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
    pub file: Option<String>,
}

#[derive(Debug, Args)]
pub struct InstallSubcommand {
    /// Configuration file path 
    pub file: Option<String>,
}

#[derive(Debug, Args)]
pub struct ApplySubcommand {
    /// Configuration file path 
    pub file: Option<String>,
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
}

fn get_config(filename: &str) {
    let content = dotcc::read_config(CONFIG_FILE_NAME).unwrap_or_else(|err| {
        eprintln!("Problem reading configuration file: {err}");
        process::exit(1);
    });
    let config = Configuration::new(&content).unwrap_or_else(|err| {
        eprintln!("Problem with creating configuration object: {err}");
        process::exit(1);
    });
}

fn parse_file(config_line: &str) {
    let home_dir = match dirs::home_dir() {
        Some(home) => home,
        None => PathBuf::from("")
    };
    let mut home_str = home_dir.into_os_string().into_string().unwrap();
    println!("Home: {}", home_str.to_string());
    let mut split = config_line.split(':');
    let mut system_path = split.next().unwrap().to_string();
    system_path.remove(0);
    home_str.push_str(&system_path);
    println!("File: {}", home_str.to_string());
    let repo_path = split.next().unwrap(); 
    println!("File: {}", repo_path);
    let system_file_handle = fs::metadata(home_str.to_string());
    let repo_file_handle = fs::metadata(repo_path);
    match system_file_handle {
       Ok(m) => println!("File {} already exists!", system_path),
       Err(error) => { 
           println!("Repository error: {}", error);
           println!("Repository is missing files referenced in config file! exiting");
           process::exit(1);
       }
    }
    
    match repo_file_handle {
       Ok(m) => println!("File {} already exists!", repo_path),
       Err(error) => { 
           println!("Repository error: {}", error);
           println!("Repository is missing files referenced in config file! exiting");
           process::exit(1);
       }
    }
}

fn run_check(subcommand: CheckSubcommand) {
    let content = dotcc::read_config(CONFIG_FILE_NAME).unwrap_or_else(|err| {
        eprintln!("Problem reading configuration file: {err}");
        process::exit(1);
    });
    let config = Configuration::new(&content).unwrap_or_else(|err| {
        eprintln!("Problem with creating configuration object: {err}");
        process::exit(1);
    });
    for line in config.files.iter() {
        parse_file(line);
    }
}

fn run_install(subcommand: InstallSubcommand) {

}

fn run_apply(subcommand: ApplySubcommand) {

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        const DEFAULT_CONFIGFILE_NAME: &str = "dotcc-config.json";
        let x = dotcc::read_config(DEFAULT_CONFIGFILE_NAME);
        let check = if let Ok(result) = x {
            true
        } else {
            false
        };
        assert!(check);
    }
}
