use hltas::HLTAS;
use hltas_cleaner::runner::{self, Config};
use std::{env, fs, process};

fn config_from_args(mut args: env::Args) -> Result<Config, &'static str> {
    let arg_count = 3;

    if args.len() < arg_count {
        // TODO better way of handling showing required arguments and environment variables
        return Err(
                "\nUsage: input_path output_path\nenvironment variables: NoBulkDupe (bool)",
            );
    }

    args.next();

    let filename = args.next();
    let output_name = args.next();
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

    if let Some(filename) = filename {
        if let Some(output_name) = output_name {
            return Ok(Config {
                file_path: filename,
                output_path: output_name,
                remove_dupe_framebulks,
            });
        }
    }

    // TODO more specific error
    Err("Unable to get required args")
}

fn main() {
    let config = config_from_args(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // not sure if I can get around not living long enough problem
    let content = fs::read_to_string(&config.file_path).unwrap_or_else(|err| {
        eprintln!("Problem opening file: {}", err);
        process::exit(1);
    });
    let mut hltas = HLTAS::from_str(&content).unwrap_or_else(|err| {
        eprintln!("Problem parsing hltas file: {}", err);
        process::exit(1);
    });

    if let Err(e) = runner::run(config, &mut hltas) {
        eprintln!("Error cleaning up, {}", e);
        process::exit(1);
    }
}
