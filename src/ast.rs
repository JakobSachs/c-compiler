#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Func>,
}

#[derive(Debug)]
pub struct Func {
    pub return_type: Type,
    pub name: String,
    pub block_items: Vec<Statement>,
    pub params: Vec<FuncParam>,
}

#[derive(Debug)]
pub struct FuncParam {
    pub param_type: Type,
    pub param_name: String,
}

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Void,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Const(i32),
    Var(String),
    Unary(UnaryOp, Box<Expr>),
    Binary(BinaryOp, Box<Expr>, Box<Expr>),
    Group(Box<Expr>),
    Assignment(String, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Negate,
    BitwiseNegate,
    Negative,
}
#[derive(Debug, Clone)]
pub enum BinaryOp {
    // Arithmetic operators
    Add,
    Subtract,
    Multiply,
    Divide,
    // Comparison operators
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    // Logical operators
    LogicalAnd,
    LogicalOr,
}

#[derive(Debug)]
pub enum Statement {
    Return(Expr),
    Expr(Expr),
    Declare(Type, String, Option<Expr>),
    If(Expr, Box<Statement>, Option<Box<Statement>>),
    Compound(Vec<Statement>),
}
