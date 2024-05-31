use std::error::Error;

pub struct Config {
    pub query: String,
    pub filepath: String,
}

impl Config {
    pub fn from(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filepath = args[2].clone();
        Ok(Config { query, filepath })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string(&config.filepath)?;
    println!("With text:\n{}", contents);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_from_valid_args() {
        let args = vec![
            String::from("sg"),
            String::from("query"),
            String::from("filepath"),
        ];
        let config = Config::from(&args).unwrap();
        assert_eq!(config.query, "query");
        assert_eq!(config.filepath, "filepath");
    }

    #[test]
    fn config_from_invalid_args() {
        let args = vec![String::from("sg")];
        let config = Config::from(&args);
        assert!(config.is_err());
    }
}
