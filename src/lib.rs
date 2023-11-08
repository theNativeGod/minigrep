use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensetive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    if results.len() == 0 {
        println!("\x1b[38;5;208mNo matches found\x1b[0m");
    } else if results.len() == 1 {
        println!("\x1b[38;5;208m1 match found\x1b[0m");
    } else {
        println!("\x1b[38;5;208m{} matches found\x1b[0m", results.len());
    }

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let mut ignore_case = env::var("IGNORE_CASE").is_ok();

        for word in args {
            if word == "--igc" {
                ignore_case = true;
            }
        }

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn one_result() {
//         let query = "safe";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.";

//         assert_eq!(vec!["safe, fast, productive."], search(query, contents));
//     }

//     #[test]
//     fn case_insensitive() {
//         let query = "rUsT";
//         let contents = "\
//         Rust:
//         safe, fast, productive.
//         Pick three.
//         Trust me.";
//         assert_eq!(
//             vec!["Rust:", "Trust me."],
//             search_case_insensetive(query, contents)
//         );
//     }
// }

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensetive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line.trim());
        }
    }
    results
}
