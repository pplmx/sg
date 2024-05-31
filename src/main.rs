use std::{env, process};

use sg::Config;

fn main() {
    let config = Config::from(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = sg::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
