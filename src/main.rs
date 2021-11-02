use hltas::HLTAS;
use hltas_cleaner::Config;
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::from_args(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // not sure if I can get around not living long enough problem
    let content = fs::read_to_string(&config.filename).unwrap_or_else(|err| {
        println!("Problem opening file: {}", err);
        process::exit(1);
    });
    let mut hltas = HLTAS::from_str(&content).unwrap_or_else(|err| {
        println!("Problem parsing hltas file: {}", err);
        process::exit(1);
    });

    if let Err(e) = hltas_cleaner::run(config, &mut hltas) {
        println!("Error cleaning up, {}", e);
        process::exit(1);
    }
}
