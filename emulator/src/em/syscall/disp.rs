use crate::Display;

const BDISP_ALLCLR_VRAM: u32 = 0x272;
const BDISP_PUTDD_VRAM: u32 = 0x25f;

pub fn is_syscall(id: u32) -> bool {
    match id {
        BDISP_ALLCLR_VRAM | BDISP_PUTDD_VRAM => true,
        _ => false,
    }
}

pub fn handle_syscall<D>(
    id: u32,
    param_1: u32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    display: &mut D,
) where
    D: Display,
{
    match id {
        BDISP_ALLCLR_VRAM => {
            println!("Bdisp_AllClr_VRAM System-Call");
            display.clear_vram();
        }
        BDISP_PUTDD_VRAM => {
            println!("Bdisp_PutDD_VRAM System-Call");
            display.display_vram();
        }
        _ => panic!("Unknown display syscall"),
    }
}
