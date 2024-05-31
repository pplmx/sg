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
    let contents = std::fs::read_to_string(&config.filepath)?;
    let results = search(&config, &contents);
    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(cfg: &'a Config, contents: &'a str) -> impl Iterator<Item = &'a str> + 'a {
    contents.lines().filter(move |line| {
        if cfg.is_case_sensitive {
            line.contains(&cfg.query)
        } else {
            line.to_lowercase().contains(&cfg.query.to_lowercase())
        }
    })
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
        let cfg = Config {
            query: String::from("safe"),
            filepath: String::from(""),
            is_case_sensitive: true,
        };
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        let results: Vec<_> = search(&cfg, contents).collect();
        assert_eq!(vec!["safe, fast, productive."], results);
    }

    #[test]
    fn search_multiple_results() {
        let cfg = Config {
            query: String::from("the"),
            filepath: String::from(""),
            is_case_sensitive: true,
        };
        let contents = "\
The a
The b
the c
the d";
        let results: Vec<_> = search(&cfg, contents).collect();
        assert_eq!(vec!["the c", "the d"], results);
    }

    #[test]
    fn search_no_results() {
        let cfg = Config {
            query: String::from("begin"),
            filepath: String::from(""),
            is_case_sensitive: true,
        };
        let contents = "The end.";
        let results: Vec<_> = search(&cfg, contents).collect();
        assert_eq!(Vec::<&str>::new(), results);
    }

    #[test]
    fn case_sensitive() {
        let cfg = Config {
            query: String::from("the"),
            filepath: String::from(""),
            is_case_sensitive: true,
        };
        let contents = "The a\nThe b\nthe c\nthe d";
        let results: Vec<_> = search(&cfg, contents).collect();
        assert_eq!(vec!["the c", "the d"], results);
    }

    #[test]
    fn case_insensitive() {
        let cfg = Config {
            query: String::from("the"),
            filepath: String::from(""),
            is_case_sensitive: false,
        };
        let contents = "The a\nThe b\nthe c\nthe d";
        let results: Vec<_> = search(&cfg, contents).collect();
        assert_eq!(vec!["The a", "The b", "the c", "the d"], results);
    }
}
