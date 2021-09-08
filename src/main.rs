//! A simple implementation of grep
//!
//! # Usage
//!
//! minigrep_eoin QUERY PATH [--case-insensitive]

use std::env;
use std::process;

use minigrep_eoin::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep_eoin::run(config) {
        eprintln!("Application error: {}", e );
        process::exit(1);
    }
}
