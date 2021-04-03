use std::collections::HashMap;

use super::ir;

mod expression;
mod function;
mod internal;
mod statement;
mod syscall;

pub type Offsets = HashMap<String, u32>;
pub type Functions = HashMap<String, ir::Function>;

fn initial_main_jump(result: &mut Vec<u8>) {
    // Storing the First byte
    result.push(0xe2);
    result.push(0);
    // Shift r2 one byte left and add the second byte
    result.push(0x42);
    result.push(0x18);
    result.push(0x72);
    result.push(0);
    // Shift r2 one byte left and add the third byte
    result.push(0x42);
    result.push(0x18);
    result.push(0x72);
    result.push(0);
    // Shift r2 one byte left and add the third byte
    result.push(0x42);
    result.push(0x18);
    result.push(0x72);
    result.push(0);

    // JMP
    result.push(0x42);
    result.push(0x2b);

    // Noop
    result.push(0x00);
    result.push(0x09);
}
fn fixup_main_jump(result: &mut Vec<u8>, main_offset: u32) {
    let target_bytes = main_offset.to_be_bytes();
    result[1] = target_bytes[0];
    result[5] = target_bytes[1];
    result[9] = target_bytes[2];
    result[13] = target_bytes[3];
}

fn print_instructions(instr: &[u8]) {
    for window in instr.chunks(2) {
        println!("{:02x}{:02x}", window[0], window[1]);
    }
}

// TODO
pub fn generate(mut funcs: Vec<ir::Function>) -> Vec<u8> {
    let mut result = Vec::new();

    // The Jump to main
    initial_main_jump(&mut result);

    let mut functions = HashMap::<String, ir::Function>::new();
    for tmp in funcs.drain(..) {
        functions.insert(tmp.0.clone(), tmp);
    }

    let main_func = functions.get("main").unwrap();

    let mut offsets = HashMap::new();
    function::generate(main_func, &mut result, &mut offsets, &functions);

    fixup_main_jump(&mut result, *offsets.get("main").unwrap());

    print_instructions(&result);
    println!("Offsets: {:?}", offsets);

    result
}
