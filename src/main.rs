use std::{env, process};

use sg::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::from(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filepath);

    if let Err(e) = sg::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

