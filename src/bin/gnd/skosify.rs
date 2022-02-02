use std::path::PathBuf;

use clap::Parser;

use crate::config::Config;
use crate::CliResult;

const DEFAULT_FILTER: &str = "002@.0 =~ '^T[bfgpsu][1z]$'";

#[derive(Parser, Debug)]
pub(crate) struct SkosifyArgs {
    #[clap(long, short)]
    pub(crate) filter: Option<String>,

    #[clap(required = false, parse(from_os_str))]
    pub(crate) paths: Vec<PathBuf>,
}

pub(crate) fn run(config: &Config, args: &SkosifyArgs) -> CliResult<()> {
    let filter = args
        .filter
        .as_deref()
        .or(config.concept.filter.as_deref())
        .unwrap_or(DEFAULT_FILTER);

    println!("filter = {:?}", filter);
    Ok(())
}
