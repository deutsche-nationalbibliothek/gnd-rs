use crate::config::Config;
use crate::CliResult;
use std::path::PathBuf;

pub(crate) fn run(_paths: &[PathBuf], config: &Config) -> CliResult<()> {
    println!("config = {:?}", config);
    Ok(())
}
