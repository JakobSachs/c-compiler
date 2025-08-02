use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar);
pub mod ast;


fn main() {
    let path = std::env::args_os().nth(1);
    let path = match path {
        Some(path) => path,
        None => {
            eprintln!("Error: No input file specified");
            std::process::exit(1);
        }
    };
    let input = std::fs::read_to_string(&path);
    let input = match input {
        Ok(input) => input,
        Err(err) => {
            eprintln!(
                "Error: Failed to read file '{}': {}",
                path.as_os_str().to_string_lossy(),
                err
            );
            std::process::exit(1);
        }
    };

    match grammar::ProgramParser::new().parse(&input) {
        Ok(output) => {
            dbg!(output);
        }
        Err(err) => {
            eprintln!("error during parsing: {}", err);
        }
    };
}
