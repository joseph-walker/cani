#![allow(dead_code)]

mod subcommands;
mod errors;
mod data;

use std::process;
use clap::{Parser, Subcommand};

use crate::errors::Error;
use crate::subcommands::sync::sync;
use crate::subcommands::ls::ls;

const STORAGE_DIRECTORY: &str = ".cani";
const STORAGE_FILE: &str = "caniuse.json";

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    subcommand: Cmds
}

#[derive(Subcommand, Debug)]
enum Cmds {
    Sync,
    Ls
}

fn main() {
    let cli = Cli::parse();

    let command_result = match cli.subcommand {
        Cmds::Sync => sync(),
        Cmds::Ls => ls()
    };

    match command_result {
        Ok(()) => process::exit(0),
        Err(Error::FetchError) => todo!(),
        Err(Error::ParseError) => todo!(),
        Err(Error::WriteError) => todo!(),
        Err(Error::ReadError) => todo!(),
    }
}
