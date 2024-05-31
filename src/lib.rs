use std::env;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filepath: String,
    pub is_case_sensitive: bool,
}

impl Config {
    pub fn from(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // Skip the first argument, which is the program name.
        args.next();

        Ok(Config {
            query: args.next().ok_or("Missing query string")?,
            filepath: args.next().ok_or("Missing file path")?,
            is_case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string(config.filepath)?;

    let search = if config.is_case_sensitive {
        search_sensitive
    } else {
        search_insensitive
    };

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
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
        let config = Config::from(args.into_iter()).unwrap();
        assert_eq!(config.query, "query");
        assert_eq!(config.filepath, "filepath");
    }

    #[test]
    fn config_from_invalid_args() {
        let args = vec![String::from("sg")];
        let config = Config::from(args.into_iter());
        assert!(config.is_err());
    }

    #[test]
    fn search_one_result() {
        let query = "safe";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_sensitive(query, contents)
        );
    }

    #[test]
    fn search_multiple_results() {
        let query = "the";
        let contents = "\
The a
The b
the c
the d";

        assert_eq!(vec!["the c", "the d"], search_sensitive(query, contents));
    }

    #[test]
    fn search_no_results() {
        let query = "fox";
        let contents = "The end.";

        assert_eq!(Vec::<&str>::new(), search_sensitive(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "the";
        let contents = "The a\nThe b\nthe c\nthe d";

        assert_eq!(vec!["the c", "the d"], search_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "the";
        let contents = "The a\nThe b\nthe c\nthe d";

        assert_eq!(
            vec!["The a", "The b", "the c", "the d"],
            search_insensitive(query, contents)
        );
    }
}
