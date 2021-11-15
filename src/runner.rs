use std::{error::Error, fs::File};

use crate::cleaners;
use hltas::HLTAS;

pub fn run(config: Config, hltas: &mut HLTAS) -> Result<(), Box<dyn Error>> {
    if config.remove_dupe_framebulks {
        cleaners::no_dupe_framebulks(hltas);
    }

    let file = File::create(config.output_path)?;
    hltas.to_writer(file)?;

    Ok(())
}

pub struct Config {
    pub file_path: String,
    pub output_path: String,
    pub remove_dupe_framebulks: bool,
}
