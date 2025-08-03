use std::io::Write;

use compiler::error::CompilerError;
use compiler::generate::CodeGenerator;
use compiler::grammar;
use compiler::semantics::SemanticAnalyzer;

fn main() -> Result<(), CompilerError> {
    let path = std::env::args_os().nth(1);
    let path = match path {
        Some(path) => path.into_string().map_err(|_| CompilerError::NoInputFile)?,
        None => {
            return Err(CompilerError::NoInputFile);
        }
    };
    let input = std::fs::read_to_string(&path)?;

    // compute output-path by stripping '.c'
    let idx = path.rfind('.').unwrap_or(path.len());
    let mut output_path = path[0..idx].to_string();
    output_path.push_str(".s");

    // parse and generate
    let programm = grammar::ProgramParser::new()
        .parse(&input)
        .map_err(|e| CompilerError::Parse(format!("{:?}", e)))?;

    // semantic analysis checks
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&programm)?;

    let mut generator = CodeGenerator::new();
    generator.generate(&programm);
    let out = generator.output();
    let mut output_file = std::fs::File::create(output_path)?;
    write!(output_file, "{}", out)?;
    Ok(())
}
