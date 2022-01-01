pub mod cleaners;

use std::{error::Error, fs::File};

pub use cleaners::*;
use hltas::HLTAS;
use clap::Parser;

#[cfg(test)]
mod tests;

pub fn run(config: Config, hltas: &mut HLTAS) -> Result<(), Box<dyn Error>> {
    if config.remove_dupe_framebulks {
        cleaners::no_dupe_framebulks(hltas);
    }

    let file = File::create(config.output_path)?;
    hltas.to_writer(file)?;

    Ok(())
}

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Config {
    #[clap(short, long)]
    pub input_path: String,
    #[clap(short, long)]
    pub output_path: String,
    #[clap(short, long)]
    pub remove_dupe_framebulks: bool,
}
