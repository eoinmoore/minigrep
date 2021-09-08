use std::error::Error;
use std::env;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
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

//     fn args_base_error(args: env::Args, expectedError: &str) {
//         match Config::new(args) {
//             Ok(config) => {
//                 if args.len() < 3 {
//                     panic!("Config should require at least three arguments.");
//                 };
//                 assert_eq!(config.query, args[1]);
//                 assert_eq!(config.filename, args[2]);
//             },
//             Err(e) => {
//                 if args.len() < 3 {
//                     assert_eq!(e, "not enough arguments");
//                 } else {
//                     panic!("Config should not fail when passed \
//                             three or more arguments.");
//                 }
//             }
//         };
//     }
// 
//     fn args_base(args: env::Args) {
//         args_base_error(args, "");
//     }
// 
//     #[test]
//     fn not_enough_args_1() {
//         let args = env::Args::new([]);
//         args_base_error(args, "Didn't get a query string");
//     }
// 
//     #[test]
//     fn not_enough_args_2() {
//         let args = env::Args::new(['a'.to_string()]);
//         args_base_error(args, "Didn't get a query string");
//     }
// 
//     #[test]
//     fn not_enough_args_3() {
//         let args = env::Args::new(['a'.to_string(), 'b'.to_string()]);
//         args_base_error(args, "Didn't get a file name");
//     }
// 
//     #[test]
//     fn enough_args_1() {
//         let args = env::Args::new(['a'.to_string(), 'b'.to_string(),
//                                    'c'.to_string()]);
//         args_base(args);
//     }
// 
//     #[test]
//     fn enough_args_2() {
//         let args = env::Args::new(['a'.to_string(), 'b'.to_string(), 'c'.to_string(),
//                                    'd'.to_string(), 'e'.to_string() ]);
//         args_base(args)
//     }
// 
//     #[test]
//     fn run_non_existent() {
//         let config = Config::new(
//             env::Args::new(["a".to_string(), "b".to_string(),
//                             "non_existent_file.txt".to_string()])).unwrap();
//         if let Ok(_) = run(config) {
//             panic!("Should panic.");
//         }
//     }
// 
//     #[test]
//     fn run_existent() {
//         let config = Config::new(
//             env::Args::new(["a".to_string(), "b".to_string(),
//                            "poem.txt".to_string()])).unwrap();
//         if let Err(e) = run(config) {
//             panic!("Panicked even though the file should exist: {}", e);
//         }
//     }

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
