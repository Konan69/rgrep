use rgrep::Config;
use std::{env, process};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {}", err);
        process::exit(1)
    });

    println!("searching for {}", config.query);
    println!("in file {}", config.filename);

    if let Err(e) = rgrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1)
    }
}
