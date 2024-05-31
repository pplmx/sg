use std::env;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filepath: String,
    pub is_case_sensitive: bool,
}

impl Config {
    pub fn from(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filepath = args[2].clone();

        // is_err() returns true if the environment variable is not set,
        // which means the search should be case-sensitive.
        let is_case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filepath, is_case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string(config.filepath)?;

    let search = if config.is_case_sensitive {
        search
    } else {
        search_case_insensitive
    };

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
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

    #[test]
    fn search_one_result() {
        let query = "safe";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn search_multiple_results() {
        let query = "the";
        let contents = "\
The a
The b
the c
the d";

        assert_eq!(vec!["the c", "the d"], search(query, contents));
    }

    #[test]
    fn search_no_results() {
        let query = "fox";
        let contents = "The end.";

        assert_eq!(Vec::<&str>::new(), search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "the";
        let contents = "The a\nThe b\nthe c\nthe d";

        assert_eq!(vec!["the c", "the d"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "the";
        let contents = "The a\nThe b\nthe c\nthe d";

        assert_eq!(vec!["The a", "The b", "the c", "the d"], search_case_insensitive(query, contents));
    }
}
