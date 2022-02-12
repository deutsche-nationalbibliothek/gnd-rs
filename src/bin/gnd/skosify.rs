use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

use clap::Parser;
use gnd::{Concept, Config, SynKind};
use pica::matcher::{MatcherFlags, RecordMatcher};
use pica::ReaderBuilder;
use sophia::graph::inmem::LightGraph;
use sophia::graph::MutableGraph;
use sophia::iri::Iri;
use sophia::ns::rdf;
use sophia::prefix::Prefix;
use sophia::serializer::turtle::{TurtleConfig, TurtleSerializer};
use sophia::term::literal::Literal;
use sophia_api::serializer::TripleSerializer;

use crate::{cli_flag, cli_option, CliError, CliResult};

const DEFAULT_FILTER: &str = "002@.0 =~ '^T[bfgpsu][1-7z]$'";

pub(crate) mod skos {
    namespace!(
        "http://www.w3.org/2004/02/skos/core#",
        Concept,
        prefLabel,
        altLabel,
        hiddenLabel,
        broader,
        related
    );
}

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

    #[clap(long, short)]
    pub(crate) output: Option<String>,

    #[clap(required = false, parse(from_os_str))]
    pub(crate) paths: Vec<PathBuf>,
}

pub(crate) fn run(config: &Config, args: &SkosifyArgs) -> CliResult<()> {
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

    let mut graph = LightGraph::new();

    for filename in &args.paths {
        let mut reader = ReaderBuilder::new()
            .skip_invalid(skip_invalid)
            .from_path(filename)?;

        for result in reader.records() {
            let record = result?;

            if filter.is_match(&record, &matcher_flags) {
                let concept = Concept::from_record(&record, config)?;
                let subj = Iri::new(concept.uri()).unwrap();

                graph.insert(&subj, &rdf::type_, &skos::Concept).unwrap();

                for synonym in concept.synset() {
                    let literal =
                        Literal::<Box<str>>::new_lang(synonym.label(), "de")
                            .unwrap();
                    match *synonym.kind() {
                        SynKind::Preferred => {
                            graph
                                .insert(&subj, &skos::prefLabel, &literal)
                                .unwrap();
                        }
                        SynKind::Alternative => {
                            graph
                                .insert(&subj, &skos::altLabel, &literal)
                                .unwrap();
                        }
                        SynKind::Hidden => {
                            graph
                                .insert(&subj, &skos::hiddenLabel, &literal)
                                .unwrap();
                        }
                    }
                }
            }
        }
    }

    let prefixes = [
        (
            Prefix::new_unchecked("gnd"),
            Iri::new_unchecked("http://d-nb.info/gnd/"),
        ),
        (
            Prefix::new_unchecked("skos"),
            Iri::new_unchecked("http://www.w3.org/2004/02/skos/core#"),
        ),
        (
            Prefix::new_unchecked("rdf"),
            Iri::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#"),
        ),
    ];

    let config = TurtleConfig::new()
        .with_pretty(true)
        .with_prefix_map(&prefixes[..]);
    let mut ser = TurtleSerializer::new_with_config(writer, config);
    ser.serialize_graph(&graph).unwrap();

    Ok(())
}
