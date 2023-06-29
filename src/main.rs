#![allow(dead_code)]

mod data;
mod errors;
mod subcommands;

use clap::{Parser, Subcommand};
use std::process;

use crate::errors::Error;
use crate::subcommands::ls::ls;
use crate::subcommands::sync::sync;
use crate::subcommands::r#use::r#use;

const STORAGE_DIRECTORY: &str = ".cani";
const STORAGE_FILE: &str = "caniuse.json";

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    subcommand: Cmds,
}

#[derive(Parser, Debug)]
struct UseArgs {
    feature: String,
}

#[derive(Subcommand, Debug)]
enum Cmds {
    Sync,
    Ls,
    Use(UseArgs),
}

fn main() {
    let cli = Cli::parse();

    let command_result = match cli.subcommand {
        Cmds::Sync => sync(),
        Cmds::Ls => ls(),
        Cmds::Use(use_args) => r#use(use_args.feature),
    };

    match command_result {
        Ok(()) => process::exit(0),
        Err(Error::FetchError) => todo!(),
        Err(Error::ParseError) => todo!(),
        Err(Error::WriteError) => todo!(),
        Err(Error::ReadError) => todo!(),
    }
}
