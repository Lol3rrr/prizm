use super::ir;

// TODO
pub fn generate(funcs: Vec<ir::Function>) -> Vec<u8> {
    println!("Generating-IR:");
    for func in funcs.iter() {
        func.pretty_print();
    }

    let mut result = Vec::new();

    result
}
