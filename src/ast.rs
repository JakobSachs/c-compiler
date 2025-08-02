#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Func>,
}

#[derive(Debug)]
pub struct Func {
    pub return_type: Type,
    pub name: String,
    pub statement: Statement,
    pub params: Vec<FuncParam>,
}

#[derive(Debug)]
pub struct FuncParam {
    pub param_type: Type,
    pub param_name: String,
}

#[derive(Debug)]
pub enum Type {
    Int,
    Void,
}

#[derive(Debug)]
pub enum Expr {
    Const(i32),
    Unary(UnaryOp, Box<Expr>),
    Binary(BinaryOp, Box<Expr>, Box<Expr>),
    Group(Box<Expr>),
}

#[derive(Debug)]
pub enum UnaryOp {
    Negate,
    BitwiseNegate,
    Negative,
}
#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Multiply,
    Divide,
    Subtract,
}

#[derive(Debug)]
pub struct Statement {
    pub return_value: Expr,
}
