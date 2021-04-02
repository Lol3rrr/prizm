use super::ir::{self, Statement};

mod expression;
mod syscall;

fn generate_statement(statement: &ir::Statement) -> Vec<u8> {
    match statement {
        Statement::DerefAssignment(name, exp) => {
            let mut result = Vec::new();

            println!("Deref-Assign: {} = {:?}", name, exp);

            // Evaluate the expression first
            result.append(&mut expression::generate(exp));

            // TODO
            // Actually assign the resulting value to the variable

            result
        }
        Statement::Return(exp) => {
            let mut result = Vec::new();

            result.append(&mut expression::generate(exp));

            // The actual return call
            result.push(0x00);
            result.push(0x0b);

            // Noop because the return is a delayed branch instruction
            result.push(0x00);
            result.push(0x09);

            result
        }
        Statement::SingleExpression(exp) => expression::generate(exp),
        _ => {
            println!("Unexpected: {:?}", statement);
            Vec::new()
        }
    }
}

fn generate_function(func: &ir::Function) -> Vec<u8> {
    func.pretty_print();

    let mut result = Vec::new();

    for statement in func.3.iter() {
        result.append(&mut generate_statement(statement));
    }

    println!("Result: {:?}", result);

    result
}

// TODO
pub fn generate(funcs: Vec<ir::Function>) -> Vec<u8> {
    let mut result = Vec::new();

    println!("Generating-IR:");
    for func in funcs.iter() {
        result.append(&mut generate_function(func));
    }

    println!("Final-Result: {:?}", result);

    result
}
