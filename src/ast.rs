use std::fmt::{Debug, Error, Formatter};

pub struct Program {
    functions: Vec<Func>,
}

pub struct Func {
    return_type: Type,
    name: String,
    statement: Statement,
}

pub enum Type {
    Int,
    Void,
}

pub struct Statement {
    return_value: i32,
}


