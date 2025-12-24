use std::env;
use std::fs;
use std::error::Error;

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
    let mut res = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    res
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut res = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line);
        }
    }
    res
}

pub struct Input {
    pub query: String,
    pub file_path: String,
    pub case: bool,
}

impl Input {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Fuck you give enough things to play");
        }
        let query = &args[1];
        let file_path = &args[2];
        let case = env::var("CASE").is_ok();
        Ok(Input {
            query: query.clone(),
            file_path: file_path.clone(),
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
