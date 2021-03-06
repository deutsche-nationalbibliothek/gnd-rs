#[macro_use]
extern crate sophia_api;

use std::{fmt, io, process};

use clap::Parser;

mod cli;
mod macros;
mod skosify;
mod tabulate;

use cli::{Cli, Commands};
use gnd::Config;

pub type CliResult<T> = Result<T, CliError>;

#[derive(Debug)]
pub enum CliError {
    Pica(pica::Error),
    Gnd(gnd::Error),
    Io(io::Error),
    Csv(csv::Error),
    Other(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::Pica(ref e) => e.fmt(f),
            CliError::Gnd(ref e) => e.fmt(f),
            CliError::Io(ref e) => e.fmt(f),
            CliError::Csv(ref e) => e.fmt(f),
            CliError::Other(ref s) => f.write_str(s),
        }
    }
}

impl From<gnd::Error> for CliError {
    fn from(err: gnd::Error) -> Self {
        CliError::Gnd(err)
    }
}

impl From<pica::Error> for CliError {
    fn from(err: pica::Error) -> Self {
        CliError::Pica(err)
    }
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> Self {
        CliError::Io(err)
    }
}

impl From<csv::Error> for CliError {
    fn from(err: csv::Error) -> Self {
        CliError::Csv(err)
    }
}

fn main() {
    let args = Cli::parse();

    let config: Config = match &args.config {
        Some(filename) => Config::from_file(filename).unwrap(),
        None => Config::default(),
    };

    let result = match args.command {
        Commands::Skosify(args) => skosify::run(&config, &args),
        Commands::Tabulate(args) => tabulate::run(&config, &args),
    };

    match result {
        Ok(()) => process::exit(0),
        Err(CliError::Io(ref err))
            if err.kind() == io::ErrorKind::BrokenPipe =>
        {
            process::exit(0); // cov:excl-line
        }
        Err(CliError::Io(err)) => {
            eprintln!("IO Error: {}", err);
            process::exit(1);
        }
        Err(CliError::Gnd(err)) => {
            eprintln!("gnd: {}", err);
            process::exit(1);
        }
        Err(CliError::Pica(err)) => {
            eprintln!("pica: {}", err);
            process::exit(1);
        }
        Err(CliError::Csv(err)) => {
            eprintln!("csv: {}", err);
            process::exit(1);
        }
        Err(CliError::Other(err)) => {
            eprintln!("error: {}", err);
            process::exit(1);
        }
    }
}
