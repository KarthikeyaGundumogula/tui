use std::fs;
use std::error::Error;


pub fn run(input: Input) -> Result<(), Box<dyn Error>>{
    let data = fs::read_to_string(input.file_path)?;
    println!("{data}");
    Ok(())
}

pub struct Input {
    pub query: String,
    pub file_path: String,
}

impl Input {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Fuck you give enough things to play");
        }
        let query = &args[1];
        let file_path = &args[2];
        Ok(Input {
            query: query.clone(),
            file_path: file_path.clone(),
        })
    }
}