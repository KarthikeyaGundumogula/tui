use std::env;
use std::error::Error;
use std::fs;

pub fn run(input: Input) -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string(input.file_path)?;
    if input.case {
        for line in search(&input.query, &data) {
            println!("{line}");
        }
    } else {
        for line in search_insensitive(&input.query, &data) {
            println!("{line}");
        }
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|s| s.to_lowercase() == query)
        .collect()
}

pub struct Input {
    pub query: String,
    pub file_path: String,
    pub case: bool,
}

impl Input {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Input, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let case = env::var("CASE").is_ok();

        Ok(Input {
            query,
            file_path,
            case,
        })
    }
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
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me";

        assert_eq!(
            vec!["Rust:", "Trust me"],
            search_insensitive(query, contents)
        );
    }
}
