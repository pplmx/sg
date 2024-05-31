use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filepath);

    let contents = std::fs::read_to_string(&config.filepath)
        .expect("Something went wrong reading the file");
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not enough arguments");
        }
        let query = args[1].clone();
        let filepath = args[2].clone();
        Config { query, filepath }
    }
}

struct Config {
    query: String,
    filepath: String,
}
