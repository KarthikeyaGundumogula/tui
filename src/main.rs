use grep_tui::Input;
use std::{env, process};

fn main() {
    let input: Input = Input::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Porbelm parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = grep_tui::run(input) {
        eprintln!("Eror occured: {e}");
        process::exit(1);
    }
}
