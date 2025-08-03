use itertools::Itertools;

use crate::ast::{Expr, Program, Statement, Type};

fn pretty_print_expr(expr: &Expr) -> String {
    match expr {
        Expr::Const(value) => value.to_string(),
        Expr::Var(name) => name.clone(),
        Expr::Unary(op, expr) => {
            let op_str = match op {
                crate::ast::UnaryOp::Negate => "!",
                crate::ast::UnaryOp::BitwiseNegate => "~",
                crate::ast::UnaryOp::Negative => "-",
            };
            format!("{}{}", op_str, pretty_print_expr(expr))
        }
        Expr::Binary(op, left, right) => {
            let op_str = match op {
                crate::ast::BinaryOp::Add => "+",
                crate::ast::BinaryOp::Subtract => "-",
                crate::ast::BinaryOp::Multiply => "*",
                crate::ast::BinaryOp::Divide => "/",
                crate::ast::BinaryOp::Equal => "==",
                crate::ast::BinaryOp::NotEqual => "!=",
                crate::ast::BinaryOp::Less => "<",
                crate::ast::BinaryOp::LessEqual => "<=",
                crate::ast::BinaryOp::Greater => ">",
                crate::ast::BinaryOp::GreaterEqual => ">=",
                crate::ast::BinaryOp::LogicalAnd => "&&",
                crate::ast::BinaryOp::LogicalOr => "||",
            };
            format!(
                "({} {} {})",
                pretty_print_expr(left),
                op_str,
                pretty_print_expr(right)
            )
        }
        Expr::Group(expr) => format!("({})", pretty_print_expr(expr)),
        Expr::Assignment(var, expr) => format!("{} = {}", var, pretty_print_expr(expr)),
        Expr::Conditional(cond, then_expr, else_expr) => format!(
            "({} ? {} : {})",
            pretty_print_expr(cond),
            pretty_print_expr(then_expr),
            pretty_print_expr(else_expr)
        ),
    }
}

fn pretty_print_statement(stmt: &Statement, indent: usize) -> String {
    let indent_str = "  ".repeat(indent);
    match stmt {
        Statement::Return(expr) => {
            format!("{}return {};", indent_str, pretty_print_expr(expr))
        }
        Statement::Expr(expr) => {
            format!("{}{};", indent_str, pretty_print_expr(expr))
        }
        Statement::Declare(type_name, var_name, init) => {
            let type_str = match type_name {
                Type::Int => "int",
                Type::Void => "void",
            };
            match init {
                Some(expr) => format!(
                    "{}{} {} = {};",
                    indent_str,
                    type_str,
                    var_name,
                    pretty_print_expr(expr)
                ),
                None => format!("{}{} {};", indent_str, type_str, var_name),
            }
        }
        Statement::If(condition, then_stmt, else_stmt) => {
            let mut result = format!("{}if ({})\n", indent_str, pretty_print_expr(condition));
            result.push_str(&pretty_print_statement(then_stmt, indent));
            if let Some(else_stmt) = else_stmt {
                result.push_str(&format!("\n{}else\n", indent_str));
                result.push_str(&pretty_print_statement(else_stmt, indent));
            }
            result
        }
        Statement::Compound(statements) => {
            let mut result = format!("{}{{\n", indent_str);
            for stmt in statements {
                result.push_str(&format!("{}\n", pretty_print_statement(stmt, indent + 1)));
            }
            format!("{}{}}}", result, indent_str)
        }
    }
}

pub fn pretty_print(program: &Program) {
    for f in program.functions.iter() {
        let return_type = match f.return_type {
            Type::Int => "int",
            Type::Void => "void",
        };
        println!("FUNC {} {}:", return_type, f.name);
        println!(
            "\tparams: ({})",
            f.params
                .iter()
                .map(|p| {
                    let param_type = match p.param_type {
                        Type::Int => "int",
                        Type::Void => "void",
                    };
                    format!("{} {}", param_type, p.param_name)
                })
                .intersperse(", ".to_string())
                .collect::<String>()
        );
        println!("\tbody:");
        for s in f.block_items.iter() {
            println!("\t{}", pretty_print_statement(s, 1));
        }
    }
}
