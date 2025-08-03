use crate::ast::{Expr, Program, Statement};
use crate::error::CompilerError;
use std::collections::HashMap;

#[derive(Default)]
pub struct SemanticAnalyzer {
    variables: HashMap<String, bool>, // true if initialized
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn analyze(&mut self, program: &Program) -> Result<(), CompilerError> {
        for function in &program.functions {
            // Reset variables for each function
            self.variables.clear();

            // Add parameters to the scope
            for param in &function.params {
                self.variables.insert(param.param_name.clone(), true);
            }

            // Analyze function body
            for statement in &function.block_items {
                self.analyze_statement(statement)?;
            }
        }
        Ok(())
    }

    fn analyze_statement(&mut self, statement: &Statement) -> Result<(), CompilerError> {
        match statement {
            Statement::Return(expr) => {
                self.analyze_expr(expr)?;
            }
            Statement::Expr(expr) => {
                self.analyze_expr(expr)?;
            }
            Statement::Declare(_type, name, init_expr) => {
                // Check for redeclaration
                if self.variables.contains_key(name) {
                    return Err(CompilerError::FunctionRedefined(name.clone()));
                }

                // Add variable to scope (initialized if it has an initializer)
                let is_initialized = init_expr.is_some();
                self.variables.insert(name.clone(), is_initialized);

                // Analyze initializer if present
                if let Some(expr) = init_expr {
                    self.analyze_expr(expr)?;
                }
            }
            Statement::If(condition, then_stmt, else_stmt) => {
                self.analyze_expr(condition)?;
                self.analyze_statement(then_stmt)?;
                if let Some(else_stmt) = else_stmt {
                    self.analyze_statement(else_stmt)?;
                }
            }
            Statement::Compound(statements) => {
                for stmt in statements {
                    self.analyze_statement(stmt)?;
                }
            }
        }
        Ok(())
    }

    fn analyze_expr(&mut self, expr: &Expr) -> Result<(), CompilerError> {
        match expr {
            Expr::Const(_) => {
                // Constants are always valid
            }
            Expr::Var(name) => {
                // Check if variable is declared
                if !self.variables.contains_key(name) {
                    return Err(CompilerError::UndeclaredVariable(name.clone()));
                }
            }
            Expr::Unary(_op, expr) => {
                self.analyze_expr(expr)?;
            }
            Expr::Binary(_op, left, right) => {
                self.analyze_expr(left)?;
                self.analyze_expr(right)?;
            }
            Expr::Group(expr) => {
                self.analyze_expr(expr)?;
            }
            Expr::Assignment(name, expr) => {
                // Check if variable is declared
                if !self.variables.contains_key(name) {
                    return Err(CompilerError::UndeclaredVariable(name.clone()));
                }
                self.analyze_expr(expr)?;
            }
            Expr::Conditional(cond, then_expr, else_expr) => {
                self.analyze_expr(cond)?;
                self.analyze_expr(then_expr)?;
                self.analyze_expr(else_expr)?;
            }
        }
        Ok(())
    }
}
