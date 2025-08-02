use std::io::Write;

use itertools::Itertools;

use compiler::ast::Expr;
use compiler::ast::Program;
use compiler::generate::generate;
use compiler::grammar;

fn pretty_print(program: &Program) {
    for f in program.functions.iter() {
        println!("FUNC {:?} {}:", f.return_type, f.name);
        println!(
            "\tparams: ({})",
            f.params
                .iter()
                .map(|p| { format!("{:?} {}", p.param_type, p.param_name) })
                .intersperse(", ".to_string())
                .collect::<String>()
        );
        println!("\tbody:");
        println!("\t\tRETURN {:?}", f.statement.return_value);
    }
}

fn main() {
    let path = std::env::args_os().nth(1);
    let path = match path {
        Some(path) => path.into_string().unwrap(),
        None => {
            eprintln!("Error: No input file specified");
            std::process::exit(1);
        }
    };
    let input = std::fs::read_to_string(&path);
    let input = match input {
        Ok(input) => input,
        Err(err) => {
            eprintln!("Error: Failed to read file '{}': {}", path, err);
            std::process::exit(1);
        }
    };

    // compute output-path by stripping '.c'
    let idx = path.rfind('.').unwrap_or(path.len());
    let mut output_path = path[0..idx].to_string();
    output_path.push_str(".s");

    // parse and generate
    match grammar::ProgramParser::new().parse(&input) {
        Ok(programm) => {
            pretty_print(&programm);
            let out = generate(&programm);
            let mut output_file = std::fs::File::create(output_path).unwrap();
            write!(output_file, "{}", out).unwrap();
        }
        Err(err) => {
            eprintln!("error during parsing: {}", err);
        }
    }
}
