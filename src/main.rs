use clap::StructOpt;
use hltas::HLTAS;
use hltas_cleaner::{Config, run};
use std::{fs, process};

fn main() {
    let config = Config::parse();

    let content = fs::read_to_string(&config.input_path).unwrap_or_else(|err| {
        eprintln!("Problem opening file: {}", err);
        process::exit(1);
    });
    let mut hltas = HLTAS::from_str(&content).unwrap_or_else(|err| {
        eprintln!("Problem parsing hltas file: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config, &mut hltas) {
        eprintln!("Error cleaning up, {}", e);
        process::exit(1);
    }
}
