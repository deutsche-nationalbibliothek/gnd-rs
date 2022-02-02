use crate::CliResult;
use std::path::PathBuf;

pub(crate) fn run(_paths: &Vec<PathBuf>) -> CliResult<()> {
    println!("skosify");
    Ok(())
}
