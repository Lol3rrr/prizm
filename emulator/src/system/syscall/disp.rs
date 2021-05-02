use crate::Display;

const BDISP_ALLCLR_VRAM: u32 = 0x272;
const BDISP_PUTDD_VRAM: u32 = 0x25f;

pub fn is_syscall(id: u32) -> bool {
    match id {
        BDISP_ALLCLR_VRAM | BDISP_PUTDD_VRAM | 0x091E | 0x02B2 | 0x01B6 | 0x0920 | 0x01A2
        | 0x0921 | 0x0276 | 0x0275 | 0x026F | 0x026E | 0x0267 | 0x1D8 | 0x1D87 | 0x1D82
        | 0x1D85 | 0x0D09 | 0x0D08 | 0x0260 | 0x0199 | 0x0194 | 0x0262 | 0x026B | 0x0263
        | 0x01C7 | 0x01BE | 0x01C0 | 0x01C4 | 0x0291 | 0x0290 | 0x1906 => true,
        _ => false,
    }
}

pub fn handle_syscall<D>(
    id: u32,
    _param_1: u32,
    _param_2: u32,
    _param_3: u32,
    _param_4: u32,
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
        // https://prizm.cemetech.net/index.php?title=Bdisp_FrameAndColor
        0x091E => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_FrameAndColor");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_AreaClr
        0x02B2 => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_AreaClr");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_AreaClr_DD_x3
        0x01B6 => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_AreaClr_DD_x3");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_ColorAndFrameSetFlags
        0x0920 => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_ColorAndFrameSetFlags");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_DDRegisterSelect
        0x01A2 => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_DDRegisterSelect");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_EnableColor
        0x0921 => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_EnableColor");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_Fill_DD
        0x0276 => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_Fill_DD");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_Fill_VRAM
        0x0275 => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_Fill_VRAM");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_GetPoint_DD
        0x026F => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_GetPoint_DD");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_GetPoint_DD_Workbench
        0x026E => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_GetPoint_DD_Workbench");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_GetPoint_VRAM
        0x0267 => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_GetPoint_VRAM");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_HeaderFill
        0x1D86 => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_HeaderFill");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_HeaderFill2
        0x1D87 => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_HeaderFill2");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_HeaderText
        0x1D82 => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_HeaderText");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_HeaderText2
        0x1D85 => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_HeaderText2");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_MMPrint
        0x0D09 => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_MMPrint");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_MMPrintRef
        0x0D08 => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_MMPrintRef");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_PutDisp_DD_stripe
        0x0260 => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_PutDisp_DD_stripe");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_SetBacklightLevel
        0x0199 => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_SetBacklightLevel");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_SetDDRegisterB
        0x0194 => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_SetDDRegisterB");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_SetPointWB_VRAM
        0x0262 => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_SetPointWB_VRAM");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_SetPoint_DD
        0x026B => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_SetPoint_DD");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_SetPoint_VRAM
        0x0263 => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_SetPoint_VRAM");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_ShapeBase
        0x01C7 => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_ShapeBase");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_ShapeBase3XVRAM
        0x01BE => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_ShapeBase3XVRAM");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_ShapeToDD
        0x01C0 => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_ShapeToDD");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_ShapeToVRAM16C
        0x01C4 => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_ShapeToVRAM16C");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_WriteGraphDD_WB
        0x0291 => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_WriteGraphDD_WB");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_WriteGraphVRAM
        0x0290 => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_WriteGraphVRAM");
        }
        // https://prizm.cemetech.net/index.php?title=Bdisp_WriteSystemMessage
        0x1906 => {
            println!("https://prizm.cemetech.net/index.php?title=Bdisp_WriteSystemMessage");
        }
        _ => panic!("Unknown display syscall"),
    }
}
