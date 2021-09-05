use std::error::Error;
use std::env;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        if args.len() > 3 && args[3].clone() == "--case-insensitive" {
            case_sensitive = false
        }

        Ok(Config {
            query: query,
            filename: filename,
            case_sensitive: case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let search_func = if config.case_sensitive {
        search
    } else {
        search_case_insensitive
    };

    for line in search_func(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matches = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            matches.push(line);
        }
    }
    matches
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matches = vec![];
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            matches.push(line);
        }
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args_base(args: &[String]) {
        match Config::new(args) {
            Ok(config) => {
                if args.len() < 3 {
                    panic!("Config should require at least three arguments.");
                };
                assert_eq!(config.query, args[1]);
                assert_eq!(config.filename, args[2]);
            },
            Err(e) => {
                if args.len() < 3 {
                    assert_eq!(e, "not enough arguments");
                } else {
                    panic!("Config should not fail when passed \
                            three or more arguments.");
                }
            }
        };
    }

    #[test]
    fn not_enough_args_1() {
        args_base(&[]);
    }

    #[test]
    fn not_enough_args_2() {
        args_base(&['a'.to_string()]);
    }

    #[test]
    fn enough_args_1() {
        args_base(&['a'.to_string(), 'b'.to_string(), 'c'.to_string() ]);
    }

    #[test]
    fn enough_args_2() {
        args_base(&['a'.to_string(), 'b'.to_string(), 'c'.to_string(),
                    'd'.to_string(), 'e'.to_string() ]);
    }

    #[test]
    fn run_non_existent() {
        let config = Config::new(
            &["a".to_string(), "b".to_string(),
              "non_existent_file.txt".to_string()]).unwrap();
        if let Ok(_) = run(config) {
            panic!("Should panic.");
        }
    }

    #[test]
    fn run_existent() {
        let config = Config::new(
            &["a".to_string(), "b".to_string(), "poem.txt".to_string()]).unwrap();
        if let Err(e) = run(config) {
            panic!("Panicked even though the file should exist: {}", e);
        }
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."],
                   search_case_insensitive(query, contents));
    }
}
