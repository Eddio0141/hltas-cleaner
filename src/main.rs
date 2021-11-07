use hltas::HLTAS;
use runner::Config;
use std::{env, fs, process};

mod runner;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::from_args(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("{}", &config.remove_dupe_framebulks);

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
