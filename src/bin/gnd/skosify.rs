use std::path::PathBuf;

use clap::Parser;
use pica::matcher::{MatcherFlags, RecordMatcher};
use pica::ReaderBuilder;

use crate::config::Config;
use crate::{CliError, CliResult};

const DEFAULT_FILTER: &str = "002@.0 =~ '^T[bfgpsu][1-7z]$'";

#[derive(Parser, Debug)]
pub(crate) struct SkosifyArgs {
    #[clap(
        help = "Only include records that match against the filter.",
        long,
        short
    )]
    pub(crate) filter: Option<String>,

    #[clap(
        help = "Whether to skip invalid PICA+ records or not.",
        long,
        short
    )]
    pub(crate) skip_invalid: bool,

    #[clap(required = false, parse(from_os_str))]
    pub(crate) paths: Vec<PathBuf>,
}

pub(crate) fn run(config: &Config, args: &SkosifyArgs) -> CliResult<()> {
    let skip_invalid = args.skip_invalid || config.concept.skip_invalid;
    // TODO: simplify expression via macro?
    let filter_str = args
        .filter
        .as_deref()
        .or(config.concept.filter.as_deref())
        .unwrap_or(DEFAULT_FILTER);

    let filter = match RecordMatcher::new(&filter_str) {
        Ok(f) => f,
        Err(_) => {
            return Err(CliError::Other(format!(
                "invalid filter: \"{}\"",
                filter_str
            )))
        }
    };

    // TODO: add matcher flags to config
    let flags = MatcherFlags::default();

    for filename in &args.paths {
        let mut reader = ReaderBuilder::new()
            .skip_invalid(skip_invalid)
            .from_path(filename)?;

        for result in reader.records() {
            let record = result?;

            if filter.is_match(&record, &flags) {
                println!("match!");
                break;
            }
        }
    }

    Ok(())
}
