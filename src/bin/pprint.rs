use std::process;

use compiler::grammar;
use compiler::pretty_print;

fn main() {
    let path = std::env::args_os().nth(1);
    let path = match path {
        Some(path) => path.into_string().unwrap(),
        None => {
            eprintln!("Error: No input file specified");
            process::exit(1);
        }
    };
    let input = std::fs::read_to_string(&path).unwrap_or_else(|err| {
        eprintln!("Error: Failed to read file '{}': {}", path, err);
        process::exit(1);
    });

    // parse and pretty print
    let program = grammar::ProgramParser::new()
        .parse(&input)
        .unwrap_or_else(|e| {
            eprintln!("Error: Failed to parse input: {:?}", e);
            process::exit(1);
        });

    pretty_print::pretty_print(&program);
}
