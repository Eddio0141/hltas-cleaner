mod cleaners;
mod tests;

use hltas::HLTAS;
use std::{error::Error, fs::File};

pub use cleaners::no_dupe_framebulks;

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

impl Config {
    pub fn from_args(args: &[String]) -> Result<Config, &str> {
        let arg_count = 4;

        if args.len() < arg_count {
            return Err("not enough arguments");
        }

        let filename = args[1].clone();
        let output_name = args[2].clone();
        let remove_dupe_framebulks = match args[3].clone().parse::<bool>() {
            Ok(b) => b,
            Err(_) => {
                return Err("failed to convert arg 4 to boolean");
            }
        };

        Ok(Config {
            file_path: filename,
            output_path: output_name,
            remove_dupe_framebulks,
        })
    }
}
