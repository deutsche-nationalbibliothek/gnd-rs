use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "gnd")]
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    #[clap(about = "Convert the integrated authority file to SKOS")]
    Skosify {
        #[clap(required = false, parse(from_os_str))]
        paths: Vec<PathBuf>,
    },
}
