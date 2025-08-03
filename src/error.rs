use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse input: {0}")]
    Parse(String), // We'll need to adapt the LALRPOP error type here

    // Example of a semantic error
    #[error("Variable '{0}' used but not declared")]
    UndeclaredVariable(String),

    // Example of a semantic error
    #[error("Function '{0}' redefined")]
    FunctionRedefined(String),

    #[error("Invalid input: No input file specified")]
    NoInputFile,
}
