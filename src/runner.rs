use std::{env, error::Error, fs::File};

use hltas::HLTAS;
use hltas_cleaner::cleaners;

pub fn run(config: Config, hltas: &mut HLTAS) -> Result<(), Box<dyn Error>> {
    if config.remove_dupe_framebulks {
        cleaners::no_dupe_framebulks(hltas);
    }
    
    if config.angle_wrap {
        cleaners::angle_wrap(hltas);
    }

    let file = File::create(config.output_path)?;
    hltas.to_writer(file)?;

    Ok(())
}

pub struct Config {
    pub file_path: String,
    pub output_path: String,
    pub remove_dupe_framebulks: bool,
    pub angle_wrap: bool,
}

impl Config {
    pub fn from_args(args: &[String]) -> Result<Config, &str> {
        let arg_count = 3;

        if args.len() < arg_count {
            // TODO better way of handling showing required arguments and environment variables
            return Err(
                "\nUsage: input_path output_path\nenvironment variables: NoBulkDupe (bool)\nAngleWrap (bool)",
            );
        }

        let filename = args[1].clone();
        let output_name = args[2].clone();
        let remove_dupe_framebulks = match env::var("NoBulkDupe") {
            Ok(mut env_var) => {
                env_var = env_var.to_lowercase();
                if let Ok(env_var) = env_var.parse::<bool>() {
                    env_var
                } else {
                    false
                }
            }
            Err(_) => false,
        };
        let angle_wrap = match env::var("AngleWrap") {
            Ok(mut env_var) => {
                env_var = env_var.to_lowercase();
                if let Ok(env_var) = env_var.parse::<bool>() {
                    env_var
                } else {
                    false
                }
            }
            Err(_) => false,
        };

        Ok(Config {
            file_path: filename,
            output_path: output_name,
            remove_dupe_framebulks,
            angle_wrap,
        })
    }
}
