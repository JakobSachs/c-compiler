use std::io::Write;

use itertools::Itertools;

use compiler::ast::Expr;
use compiler::ast::Program;
use compiler::generate::CodeGenerator;
use compiler::grammar;

fn pretty_print_expr(expr: &Expr) -> String {
    match expr {
        Expr::Const(value) => value.to_string(),
        Expr::Var(name) => name.clone(),
        Expr::Unary(op, expr) => {
            let op_str = match op {
                compiler::ast::UnaryOp::Negate => "!",
                compiler::ast::UnaryOp::BitwiseNegate => "~",
                compiler::ast::UnaryOp::Negative => "-",
            };
            format!("{}{}", op_str, pretty_print_expr(expr))
        }
        Expr::Binary(op, left, right) => {
            let op_str = match op {
                compiler::ast::BinaryOp::Add => "+",
                compiler::ast::BinaryOp::Subtract => "-",
                compiler::ast::BinaryOp::Multiply => "*",
                compiler::ast::BinaryOp::Divide => "/",
                compiler::ast::BinaryOp::Equal => "==",
                compiler::ast::BinaryOp::NotEqual => "!=",
                compiler::ast::BinaryOp::Less => "<",
                compiler::ast::BinaryOp::LessEqual => "<=",
                compiler::ast::BinaryOp::Greater => ">",
                compiler::ast::BinaryOp::GreaterEqual => ">=",
                compiler::ast::BinaryOp::LogicalAnd => "&&",
                compiler::ast::BinaryOp::LogicalOr => "||",
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
    }
}

fn pretty_print_statement(stmt: &compiler::ast::Statement, indent: usize) -> String {
    let indent_str = "  ".repeat(indent);
    match stmt {
        compiler::ast::Statement::Return(expr) => {
            format!("{}return {};", indent_str, pretty_print_expr(expr))
        }
        compiler::ast::Statement::Expr(expr) => {
            format!("{}{};", indent_str, pretty_print_expr(expr))
        }
        compiler::ast::Statement::Declare(type_name, var_name, init) => {
            let type_str = match type_name {
                compiler::ast::Type::Int => "int",
                compiler::ast::Type::Void => "void",
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
        compiler::ast::Statement::If(condition, then_stmt, else_stmt) => {
            let mut result = format!("{}if ({})\n", indent_str, pretty_print_expr(condition));
            result.push_str(&pretty_print_statement(then_stmt, indent));
            if let Some(else_stmt) = else_stmt {
                result.push_str(&format!("\n{}else\n", indent_str));
                result.push_str(&pretty_print_statement(else_stmt, indent));
            }
            result
        }
        compiler::ast::Statement::Compound(statements) => {
            let mut result = format!("{}{{\n", indent_str);
            for stmt in statements {
                result.push_str(&format!("{}\n", pretty_print_statement(stmt, indent + 1)));
            }
            format!("{}{}}}", result, indent_str)
        }
    }
}

fn pretty_print(program: &Program) {
    for f in program.functions.iter() {
        let return_type = match f.return_type {
            compiler::ast::Type::Int => "int",
            compiler::ast::Type::Void => "void",
        };
        println!("FUNC {} {}:", return_type, f.name);
        println!(
            "\tparams: ({})",
            f.params
                .iter()
                .map(|p| {
                    let param_type = match p.param_type {
                        compiler::ast::Type::Int => "int",
                        compiler::ast::Type::Void => "void",
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

fn main() -> Result<(), std::io::Error> {
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
    let programm = grammar::ProgramParser::new().parse(&input).unwrap();
    pretty_print(&programm);
    let mut generator = CodeGenerator::new();
    generator.generate(&programm);
    let out = generator.output();
    let mut output_file = std::fs::File::create(output_path)?;
    write!(output_file, "{}", out)?;
    Ok(())
}
