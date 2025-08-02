use crate::ast::{BinaryOp, Expr, Program, UnaryOp};

pub fn generate_expr(expr: &Expr) -> String {
    // for now we assume all previous expr-values live on the stack
    // we then use x0, and x1 to pop them
    match expr {
        Expr::Const(val) => format!("\tmov x0, #{val}\n\tsub sp, sp, #16\n\tstr x0, [sp]\n"),
        Expr::Group(expr) => generate_expr(expr),
        Expr::Unary(op, expr) => {
            let mut out = generate_expr(expr);
            // pop previous result into x0
            out.push_str("\tldr x0, [sp]\n\tadd sp, sp, #16\n");
            match op {
                UnaryOp::Negative => out.push_str("\tneg\tx0, x0\n"),
                UnaryOp::BitwiseNegate => out.push_str("\tmvn\tx0, x0\n"),
                UnaryOp::Negate => out.push_str("\tcmp\tx0, #0\n\tcset\tx0, EQ\n"),
            }
            // store output onto stack
            out.push_str("\tsub sp, sp, #16\n\tstr x0, [sp]\n");
            out
        }
        Expr::Binary(op, l_expr, r_expr) => {
            let mut out = generate_expr(l_expr);
            out.push_str(&generate_expr(r_expr));

            // pop previous results into x0 and x1
            out.push_str("\tldr x1, [sp]\n\tldr x0, [sp, #16]\n\tadd sp, sp, #32\n"); //  x0 is l_expr, x1 is r_expr
            out.push_str(match op {
                BinaryOp::Add => "\tadd x0, x0, x1\n",
                BinaryOp::Subtract => "\tsub x0, x0, x1\n",
                BinaryOp::Multiply => "\tmul x0, x0, x1\n",
                BinaryOp::Divide => "\tsdiv x0, x0, x1\n",
            });
            out.push_str("\tsub sp, sp, #16\n\tstr x0, [sp]\n");
            out
        }
    }
}

pub fn generate(program: &Program) -> String {
    let mut output = String::from(".global _start\n.align 2\n\n");

    for f in program.functions.iter() {
        // function definition
        output.push_str("_start:\n");
        // content
        output.push_str(&generate_expr(&f.statement.return_value));
        output.push_str("\tldr x0, [sp]\n\tadd sp, sp, #16\n");
        output.push_str("\tret\n");
    }
    output
}
