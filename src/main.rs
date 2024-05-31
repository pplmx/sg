use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    // 如果 Result 对象是 Err，我们会调用 unwrap_or_else 方法来处理这个错误并退出程序。
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filepath);

    let contents = std::fs::read_to_string(&config.filepath)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filepath = args[2].clone();
        Ok(Config { query, filepath })
    }
}

struct Config {
    query: String,
    filepath: String,
}
