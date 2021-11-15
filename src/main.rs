use hltas::HLTAS;
use hltas_cleaner::runner::{self, Config};
use std::{env, fs, process};

fn config_from_args(args: &[String]) -> Result<Config, &str> {
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

    Ok(Config {
        file_path: filename,
        output_path: output_name,
        remove_dupe_framebulks,
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = config_from_args(&args).unwrap_or_else(|err| {
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
