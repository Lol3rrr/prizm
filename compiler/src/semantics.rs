use crate::ir;

fn validate_statement(_statement: &ir::Statement) -> bool {
    true
}

fn validate_func(func: &ir::Function) -> bool {
    for statement in func.3.iter() {
        if !validate_statement(statement) {
            return false;
        }
    }

    true
}

pub fn validate(ir: &[ir::Function]) -> bool {
    for func in ir.iter() {
        if !validate_func(func) {
            return false;
        }
    }

    true
}
