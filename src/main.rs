use std::env;
use std::fs;
use std::path::Path;
use std::process;
use std::error::Error;

use minigrep::Arguments;
use minigrep::parse_arguments;

fn main() {
    let args: Vec<String> = env::args()
        .collect();

    // two different ways to handle the Err returned from a Result
    let parsed_args = Arguments::new(&args).unwrap_or_else(|err| {
        process::exit(1);
    });

    if let Err(message) = parse_arguments(parsed_args) {
        process::exit(1);
    }
}

