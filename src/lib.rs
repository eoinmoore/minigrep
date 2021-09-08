use std::error::Error;
use std::env;
use std::fs;

#[cfg(test)]
use std::iter;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new<T>(mut args: T) -> Result<Config, &'static str>
where
    T: Iterator<Item = String>,
{
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        match args.next() {
            Some(arg) => {
                if arg == "--case-insensitive" {
                    case_sensitive = false;
                }
            },
            None => (),
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
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args_base_error<T>(args: T, expected_error: &str)
    where
        T: Iterator<Item = String>,
        T: Clone,
    {
        let args_vector: Vec<_> = args.clone().collect();
        match Config::new(args) {
            Ok(config) => {
                assert_eq!(config.query, args_vector[1]);
                assert_eq!(config.filename, args_vector[2]);
            },
            Err(e) => {
                assert_eq!(e, expected_error);
            }
        };
    }

    fn args_base<T>(args: T)
    where
        T: Iterator<Item = String>,
        T: Clone,
    {
        args_base_error(args, "");
    }

    #[test]
    fn not_enough_args_1() {
        let args = iter::empty::<String>();
        args_base_error(args, "Didn't get a query string");
    }

    #[test]
    fn not_enough_args_2() {
        let args = ['a'].iter().map(|s| s.to_string());
        args_base_error(args, "Didn't get a query string");
    }

    #[test]
    fn not_enough_args_3() {
        let args = ['a', 'b'].iter().map(|s| s.to_string());
        args_base_error(args, "Didn't get a file name");
    }

    #[test]
    fn enough_args_1() {
        let args = ['a', 'b', 'c'].iter().map(|s| s.to_string());
        args_base(args);
    }

    #[test]
    fn enough_args_2() {
        let args = ['a', 'b', 'c', 'd', 'e' ].iter().map(|s| s.to_string());
        args_base(args);
    }

    #[test]
    fn run_non_existent() {
        let config = Config::new(["a", "b", "non_existent_file.txt"].iter()
                         .map(|s| s.to_string())).unwrap();
        if let Ok(_) = run(config) {
            panic!("Should panic.");
        }
    }

    #[test]
    fn run_existent() {
        let config = Config::new( ["a", "b", "poem.txt"]
                         .iter().map(|s| s.to_string())).unwrap();
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
