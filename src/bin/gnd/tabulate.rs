use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::str::FromStr;

use clap::{Parser, Subcommand};
use csv::{Writer, WriterBuilder};
use gnd::{Concept, Config};
use pica::matcher::{MatcherFlags, RecordMatcher};
use pica::ReaderBuilder;

use crate::{cli_flag, cli_option, CliError, CliResult};

const DEFAULT_FILTER: &str = "002@.0 =~ '^T[bfgpsu][1-7z]$'";

#[derive(Subcommand, Debug)]
pub(crate) enum TabulateCommands {
    Synonyms,
}

impl FromStr for TabulateCommands {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "synonyms" => Ok(TabulateCommands::Synonyms),
            _ => Err("invalid tabulate command".to_string()),
        }
    }
}

#[derive(Parser, Debug)]
pub(crate) struct TabulateArgs {
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

    #[clap(long, short)]
    pub(crate) output: Option<String>,

    pub(crate) command: TabulateCommands,

    #[clap(required = false, parse(from_os_str))]
    pub(crate) paths: Vec<PathBuf>,
}

pub(crate) fn tabulate_synonyms(
    concept: &Concept,
    writer: &mut Writer<Box<dyn Write>>,
) {
    for synonym in concept.synset() {
        writer
            .write_record(&[
                concept.uri(),
                &synonym.kind().to_string(),
                synonym.label(),
            ])
            .unwrap();
    }
}

pub(crate) fn run(config: &Config, args: &TabulateArgs) -> CliResult<()> {
    let filter_str =
        cli_option!(args.filter, config.concept.filter, DEFAULT_FILTER);
    let skip_invalid =
        cli_flag!(args.skip_invalid, config.concept.skip_invalid);

    let matcher_flags = MatcherFlags::default();
    let filter = match RecordMatcher::new(&filter_str) {
        Ok(f) => f,
        Err(_) => {
            return Err(CliError::Other(format!(
                "invalid filter: \"{}\"",
                filter_str
            )))
        }
    };

    let writer: Box<dyn Write> = match &args.output {
        Some(filename) => Box::new(File::create(filename)?),
        None => Box::new(io::stdout()),
    };

    let mut writer = WriterBuilder::new().from_writer(writer);
    match args.command {
        TabulateCommands::Synonyms => {
            writer.write_record(&["uri", "kind", "synonym"])?;
        }
    }

    for filename in &args.paths {
        let mut reader = ReaderBuilder::new()
            .skip_invalid(skip_invalid)
            .from_path(filename)?;

        for result in reader.records() {
            let record = result?;

            if filter.is_match(&record, &matcher_flags) {
                let concept = Concept::from_record(&record, config)?;

                match args.command {
                    TabulateCommands::Synonyms => {
                        tabulate_synonyms(&concept, &mut writer)
                    }
                }
            }
        }
    }

    writer.flush()?;
    Ok(())
}
