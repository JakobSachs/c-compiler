use crate::ast::{BinaryOp, Expr, Program, Statement, Type, UnaryOp};
use std::collections::HashMap;

#[derive(Clone)]
struct VariableDef {
    address: usize,
    id: String,
    var_type: Type,
}

// TODO: handle scope
#[derive(Default)]
pub struct CodeGenerator {
    buffer: String,
    variables: HashMap<String, VariableDef>,
    stack_offset: usize,
    frame_offset: usize,  // offset from frame pointer for next variable
    label_counter: usize, // for generating unique labels
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            variables: HashMap::new(),
            stack_offset: 0,
            frame_offset: 0,
            label_counter: 0,
        }
    }

    pub fn emit_line(&mut self, code: &str) {
        self.buffer.push_str(code);
        self.buffer.push('\n');
    }

    // gets a random not used labelsofar , TODO: keep track of these
    fn get_unique_label(&mut self, prefix: &str) -> String {
        let label = format!("{}_{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }

    pub fn generate_expr(&mut self, expr: &Expr) {
        // for now we assume all previous expr-values live on the stack
        // we then use x0, and x1 to pop them
        match expr {
            Expr::Const(val) => {
                self.stack_offset += 0x10;
                self.emit_line(&format!("\tmov x0, #{val}"));
                self.emit_line("\tsub sp, sp, #0x10");
                self.emit_line("\tstr x0, [sp]");
            }
            Expr::Var(id) => {
                // load value from var-address to x0 reg
                match self.variables.get(id) {
                    Some(def) => {
                        let var_offset = def.address;
                        self.emit_line(&format!("\tldr x0, [fp, #-{var_offset}]"));
                        self.emit_line("\tsub sp, sp, #0x10");
                        self.emit_line("\tstr x0, [sp]");
                        self.stack_offset += 0x10;
                    }
                    None => panic!(),
                }
            }
            Expr::Assignment(id, expr) => {
                self.generate_expr(expr);
                // save value to var-address and return the assigned value
                match self.variables.get(id) {
                    Some(def) => {
                        // load value from stack but keep it there for return
                        let var_offset = def.address;
                        self.emit_line("\tldr x0, [sp]"); // load value into x0
                        self.emit_line(&format!("\tstr x0, [fp, #-{var_offset}]"));
                        // store to variable
                        // value remains on stack as the result of the assignment expression
                    }
                    None => panic!(),
                }
            }
            Expr::Group(expr) => {
                self.generate_expr(expr);
            }
            Expr::Unary(op, expr) => {
                self.generate_expr(expr);
                // pop previous result into x0
                self.stack_offset -= 0x10;
                self.emit_line("\tldr x0, [sp]");
                self.emit_line("\tadd sp, sp, #0x10");
                match op {
                    UnaryOp::Negative => self.emit_line("\tneg\tx0, x0"),
                    UnaryOp::BitwiseNegate => self.emit_line("\tmvn\tx0, x0"),
                    UnaryOp::Negate => {
                        self.emit_line("\tcmp\tx0, #0");
                        self.emit_line("\tcset\tx0, EQ");
                    }
                }
                // store output onto stack
                self.stack_offset += 0x10;
                self.emit_line("\tsub sp, sp, #0x10");
                self.emit_line("\tstr x0, [sp]");
            }
            Expr::Binary(op, l_expr, r_expr) => {
                match op {
                    BinaryOp::LogicalOr => {
                        // short-circuit OR: if left is true, skip right
                        let end_label = self.get_unique_label("or_end");
                        let right_label = self.get_unique_label("or_right");

                        // Evaluate left expression
                        self.generate_expr(l_expr);
                        self.emit_line("\tldr x0, [sp]"); // peek at result
                        self.emit_line("\tcmp x0, #0"); // test if true
                        self.emit_line(&format!("\tbeq {}", right_label)); // if false, evaluate right

                        // Left was true, set result to 1 and skip right
                        self.emit_line("\tmov x0, #1");
                        self.emit_line("\tstr x0, [sp]"); // store result back on stack
                        self.emit_line(&format!("\tb {}", end_label));

                        // Evaluate right expression
                        self.emit_line(&format!("{}:", right_label));
                        self.stack_offset -= 0x10;
                        self.emit_line("\tadd sp, sp, #0x10"); // pop left result
                        self.generate_expr(r_expr); // right result now on stack

                        // Convert right result to 0 or 1
                        self.emit_line("\tldr x0, [sp]");
                        self.emit_line("\tcmp x0, #0");
                        self.emit_line("\tmov x0, #0");
                        self.emit_line("\tcset x0, NE"); // set to 1 if not equal to 0
                        self.emit_line("\tstr x0, [sp]");

                        self.emit_line(&format!("{}:", end_label));
                    }
                    BinaryOp::LogicalAnd => {
                        // short-circuit AND: if left is false, skip right
                        let end_label = self.get_unique_label("and_end");
                        let right_label = self.get_unique_label("and_right");

                        // Evaluate left expression
                        self.generate_expr(l_expr);
                        self.emit_line("\tldr x0, [sp]"); // peek at result
                        self.emit_line("\tcmp x0, #0"); // test if false
                        self.emit_line(&format!("\tbne {}", right_label)); // if true, evaluate right

                        // Left was false, set result to 0 and skip right
                        self.emit_line("\tmov x0, #0");
                        self.emit_line("\tstr x0, [sp]"); // store result back on stack
                        self.emit_line(&format!("\tb {}", end_label));

                        // Evaluate right expression
                        self.emit_line(&format!("{}:", right_label));
                        self.stack_offset -= 0x10;
                        self.emit_line("\tadd sp, sp, #0x10"); // pop left result
                        self.generate_expr(r_expr); // right result now on stack

                        // Convert right result to 0 or 1
                        self.emit_line("\tldr x0, [sp]");
                        self.emit_line("\tcmp x0, #0");
                        self.emit_line("\tmov x0, #0");
                        self.emit_line("\tcset x0, NE"); // set to 1 if not equal to 0
                        self.emit_line("\tstr x0, [sp]");

                        self.emit_line(&format!("{}:", end_label));
                    }
                    _ => {
                        // Regular binary operations that need both operands
                        self.generate_expr(l_expr);
                        self.generate_expr(r_expr);

                        // pop previous results into x0 and x1
                        self.stack_offset -= 0x20;
                        self.emit_line("\tldr x1, [sp]");
                        self.emit_line("\tldr x0, [sp, #0x10]");
                        self.emit_line("\tadd sp, sp, #32"); //  x0 is l_expr, x1 is r_expr

                        // do op on x0 x1
                        match op {
                            BinaryOp::Add => self.emit_line("\tadd x0, x0, x1"),
                            BinaryOp::Subtract => self.emit_line("\tsub x0, x0, x1"),
                            BinaryOp::Multiply => self.emit_line("\tmul x0, x0, x1"),
                            BinaryOp::Divide => self.emit_line("\tsdiv x0, x0, x1"),
                            _ => {
                                let condition = match op {
                                    BinaryOp::Equal => "EQ",
                                    BinaryOp::NotEqual => "NE",
                                    BinaryOp::Greater => "GT",
                                    BinaryOp::Less => "LT",
                                    BinaryOp::GreaterEqual => "GE",
                                    BinaryOp::LessEqual => "LE",
                                    _ => unreachable!(),
                                };
                                self.emit_line("\tcmp x0, x1");
                                self.emit_line("\tmov x0, #0");
                                self.emit_line(&format!("\tcset x0, {}", condition));
                            }
                        }
                        // push res
                        self.stack_offset += 0x10;
                        self.emit_line("\tsub sp, sp, #0x10");
                        self.emit_line("\tstr x0, [sp]");
                    }
                }
            }
        }
    }

    fn generate_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Return(expr) => {
                self.generate_expr(expr);
                self.stack_offset -= 0x10;
                self.emit_line("\tldr x0, [sp]");
                self.emit_line("\tadd sp, sp, #0x10");

                // function epilogue: restore stack and frame pointer
                self.emit_line("\tmov sp, fp"); // restore stack pointer
                self.emit_line("\tldp fp, lr, [sp], #16"); // restore fp and lr, post-increment sp
                self.emit_line("\tret");
            }
            Statement::Declare(var_type, id, value) => {
                // check if already declared
                if self.variables.contains_key(id) {
                    panic!(); // TODO: proper error handling
                }

                // allocate space for the variable
                self.frame_offset += 0x10;
                self.emit_line("\tsub sp, sp, #0x10");
                self.stack_offset += 0x10;

                match value {
                    None => {
                        // variable allocated but not initialized
                    }
                    Some(expr) => {
                        // generate the initialization expression
                        self.generate_expr(expr);
                        // pop the result and store it in the variable location
                        self.emit_line("\tldr x0, [sp]");
                        self.stack_offset -= 0x10;
                        self.emit_line("\tadd sp, sp, #0x10");
                        self.emit_line(&format!("\tstr x0, [fp, #-{}]", self.frame_offset));
                    }
                }

                self.variables.insert(
                    id.clone(),
                    VariableDef {
                        id: id.clone(),
                        var_type: var_type.clone(),
                        address: self.frame_offset,
                    },
                );
            }
            Statement::Expr(e) => {
                self.generate_expr(e);
            }
        }
    }

    pub fn generate(&mut self, program: &Program) {
        self.emit_line(".global _start");
        self.emit_line(".align 2");

        for f in program.functions.iter() {
            // function definition
            self.emit_line("_start:");

            // function prologue: save old frame pointer and set up new one
            self.emit_line("\tstp fp, lr, [sp, #-16]!"); // save fp and lr, pre-decrement sp
            self.emit_line("\tmov fp, sp"); // set up frame pointer

            // content
            let mut had_return = false;
            for s in f.statements.iter() {
                if let Statement::Return(..) = s {
                    had_return = true;
                }
                self.generate_statement(s);
            }
            if !had_return {
                self.emit_line("\tmov x0, #0");
                self.emit_line("\tmov sp, fp"); // restore stack pointer
                self.emit_line("\tldp fp, lr, [sp], #16"); // restore fp and lr, post-increment sp
                self.emit_line("\tret");
            }
        }
    }

    pub fn output(self) -> String {
        self.buffer
    }
}
