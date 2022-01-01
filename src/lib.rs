pub mod cleaners;
pub use cleaners::*;

use std::{error::Error, fs::File};

use clap::Parser;
use hltas::HLTAS;

#[cfg(test)]
mod tests;

/// Runs through cleaning utilities with the given arguments.
///
/// # Arguments
/// * `config` - Configuration on what cleaners to use.
/// * `hltas` - HLTAS struct to clean up.
pub fn run(config: Config, hltas: &mut HLTAS) -> Result<(), Box<dyn Error>> {
    if config.remove_dupe_framebulks {
        cleaners::no_dupe_framebulks(hltas);
    }
    if config.remove_comments {
        cleaners::remove_comments(hltas);
    }

    let file = File::create(config.output_path)?;
    hltas.to_writer(file)?;

    Ok(())
}

#[derive(Parser, Debug)]
#[clap(about, version, author)]
/// Configuration used to run the cleaner
pub struct Config {
    #[clap(short, long)]
    pub input_path: String,
    #[clap(short, long)]
    pub output_path: String,
    #[clap(short = 'f', long)]
    pub remove_dupe_framebulks: bool,
    #[clap(short = 'c', long)]
    pub remove_comments: bool,
}
