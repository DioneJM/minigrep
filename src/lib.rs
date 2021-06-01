use std::fs;
use std::path::Path;
use std::error::Error;
use std::borrow::Borrow;

pub struct Arguments<'a> {
    query: &'a String,
    filename: &'a String,
    case_sensitive: bool
}

impl<'a> Arguments<'a> {
    pub fn new(args: &Vec<String>) -> Result<Arguments, &str> {
        if args.len() < 3 {
            return Err("Invalid arguments - You must supply a search query and filename");
        }
        let query: &String = &args[1];
        let filename: &String = &args[2];
        let mut flag_one: &String = &String::from("");

        if (args.len() > 3) {
            flag_one = &args[3];
        }

        let disable_case_sensitive = match flag_one.as_str() {
            "--ignore-case" => true,
            _ => false
        };

        Ok(Arguments {
            query,
            filename,
            case_sensitive: !disable_case_sensitive
        })
    }
}

pub fn parse_arguments(arguments: Arguments) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(Path::new(arguments.filename))?;
    println!("========== Contents ==========");
    println!("{}", contents);
    println!("==============================");
    println!("========== Search Results ==========");
    let search_results = match arguments.case_sensitive {
        true => search(arguments.query, &contents),
        false => search_insensitive(arguments.query, &contents)
    };
    for line in search_results {
        println!("{}", line);
    }
    println!("====================================");
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .map(|line| line.trim())
        .collect()
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.to_lowercase().contains(query.to_lowercase().as_str()))
        .map(|line| line.trim())
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_insensitive(query, contents)
        );
    }

}