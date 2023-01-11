#![allow(unused)]
use clap::{Parser, arg, Subcommand, Args};
use gru::Configuration;
use std::{env, process, error::Error, fs};
use serde::Deserialize;

/// Config file manager
static CONFIG_FILE_NAME: &str = "gru-settings.json";

#[derive(Debug, Parser)]
struct GruArgs {
   #[clap(subcommand)]
   action: GruCommand,
}

#[derive(Debug, Subcommand)]
pub enum GruCommand {
    /// Check differneces between files
    Check(CheckSubcommand),
    /// Install files in filesystem
    Install(InstallSubcommand),
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

// pub fn read_command(mut args: env::Args) -> Result<Command, Box<dyn Error>> {
//     let m = clap::Command::new("gru")
//         .subcommand_required(true)
//         .arg_required_else_help(true)
//         .allow_external_subcommands(true)
//         .subcommand(
//             clap::Command::new("check")
//             .about("Validates repository")
//             .arg_required_else_help(true),
//         )
//         .subcommand(
//             clap::Command::new("import")
//             .about("Imports configuration")
//             .arg_required_else_help(true),
//         ).get_matches();
//     Result::Ok(())
// }