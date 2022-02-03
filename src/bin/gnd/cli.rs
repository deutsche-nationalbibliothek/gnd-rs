use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::skosify::SkosifyArgs;

#[derive(Parser, Debug)]
#[clap(name = "gnd")]
pub(crate) struct Cli {
    #[clap(long, short, required = false, parse(from_os_str))]
    pub(crate) config: Option<PathBuf>,

    #[clap(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    #[clap(about = "Convert the integrated authority file to SKOS")]
    Skosify(SkosifyArgs),
}
