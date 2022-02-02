use std::{fmt, process};

use clap::Parser;

mod cli;
mod skosify;

use cli::{Cli, Commands};

pub type CliResult<T> = Result<T, CliError>;

#[derive(Debug)]
pub enum CliError {
    Other(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::Other(ref s) => f.write_str(s),
        }
    }
}

fn main() {
    let args = Cli::parse();

    let result = match args.command {
        Commands::Skosify { paths } => skosify::run(&paths),
    };

    match result {
        Ok(()) => process::exit(0),
        Err(CliError::Other(err)) => {
            eprintln!("error: {}", err);
            process::exit(1);
        }
    }
}
