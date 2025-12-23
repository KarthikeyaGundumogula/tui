use grep_tui::Input;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: Input = Input::build(&args).unwrap_or_else(|err| {
        println!("Porbelm parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searchign for {}", input.query);
    println!("in {}", input.file_path);
    if let Err(e) = grep_tui::run(input) {
        println!("Eror occured: {e}");
        process::exit(1);
    }
}
