use crate::{Display, Input, Memory};

mod disp;
mod fs;

const GETKEY: u32 = 0xeab;
const GETKEYWAIT_OS: u32 = 0x12bf;
const PRGM_GETKEY_OS: u32 = 0xd39;

/// Executes a single System-Call
pub fn syscall<I, D>(id: u32, memory: &mut Memory, input: &mut I, display: &mut D)
where
    I: Input,
    D: Display,
{
    let param_1 = memory.read_register(4);
    let param_2 = memory.read_register(5);
    let param_3 = memory.read_register(6);
    let param_4 = memory.read_register(7);

    match id {
        GETKEY => {
            let (key, modifier) = input.get_key();
            let key_code = key.serialize(&modifier);

            memory.write_long(param_1, key_code as u32, display);
        }
        GETKEYWAIT_OS => {
            println!("GetKeyWait_OS");
        }
        PRGM_GETKEY_OS => {
            println!("PRGM_GetKey_OS");
        }
        _ if disp::is_syscall(id) => {
            disp::handle_syscall(id, param_1, param_2, param_3, param_4, display);
        }
        _ if fs::is_syscall(id) => {
            fs::handle_syscall(id, param_1, param_2, param_3, param_4);
        }
        _ => {
            println!("Unknown Syscall: x{:04X}", id);
        }
    };
}
