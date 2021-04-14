use crate::{Display, Input, Memory};

const GETKEY: u32 = 0xeab;
const GETKEYWAIT_OS: u32 = 0x12bf;
const PRGM_GETKEY_OS: u32 = 0xd39;
const BDISP_ALLCLR_VRAM: u32 = 0x272;
const BDISP_PUTDD_VRAM: u32 = 0x25f;

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
        BDISP_ALLCLR_VRAM => {
            println!("Bdisp_AllClr_VRAM System-Call");
            display.clear_vram();
        }
        BDISP_PUTDD_VRAM => {
            println!("Bdisp_PutDD_VRAM System-Call");
            display.display_vram();
        }
        _ => {
            println!("Unknown Syscall: x{:04X}", id);
        }
    };
}
