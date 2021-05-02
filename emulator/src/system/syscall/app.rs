pub fn is_syscall(id: u32) -> bool {
    match id {
        0x0A87 | 0x0C6B | 0x0D79 | 0x1399 | 0x140A | 0x1398 | 0x1409 | 0x1397 | 0x13A7 | 0x13A6
        | 0x1384 | 0x1632 | 0x1630 | 0x1945 | 0x1A03 | 0x1E13 | 0x1E07 | 0x1E0A | 0x1E0D
        | 0x1E05 | 0x1E17 | 0x1E34 => true,
        _ => false,
    }
}

pub fn handle_syscall(id: u32) {
    match id {
        // https://prizm.cemetech.net/index.php?title=APP_EACT_StatusIcon
        0x0A87 => {
            println!("https://prizm.cemetech.net/index.php?title=APP_EACT_StatusIcon");
        }
        // https://prizm.cemetech.net/index.php?title=APP_FINANCE
        0x0C6B => {
            println!("https://prizm.cemetech.net/index.php?title=APP_FINANCE");
        }
        // https://prizm.cemetech.net/index.php?title=App_InitDlgDescriptor
        0x0D79 => {
            println!("https://prizm.cemetech.net/index.php?title=App_InitDlgDescriptor");
        }
        // https://prizm.cemetech.net/index.php?title=App_LINK_GetDeviceInfo
        0x1399 => {
            println!("https://prizm.cemetech.net/index.php?title=App_LINK_GetDeviceInfo");
        }
        // https://prizm.cemetech.net/index.php?title=App_LINK_GetReceiveTimeout_ms
        0x140A => {
            println!("https://prizm.cemetech.net/index.php?title=App_LINK_GetReceiveTimeout_ms");
        }
        // https://prizm.cemetech.net/index.php?title=App_LINK_Send_ST9_Packet
        0x1398 => {
            println!("https://prizm.cemetech.net/index.php?title=App_LINK_Send_ST9_Packet");
        }
        // https://prizm.cemetech.net/index.php?title=App_LINK_SetReceiveTimeout_ms
        0x1409 => {
            println!("https://prizm.cemetech.net/index.php?title=App_LINK_SetReceiveTimeout_ms");
        }
        // https://prizm.cemetech.net/index.php?title=App_LINK_SetRemoteBaud
        0x1397 => {
            println!("https://prizm.cemetech.net/index.php?title=App_LINK_SetRemoteBaud");
        }
        // https://prizm.cemetech.net/index.php?title=App_LINK_Transmit
        0x13A7 => {
            println!("https://prizm.cemetech.net/index.php?title=App_LINK_Transmit");
        }
        // https://prizm.cemetech.net/index.php?title=App_LINK_TransmitInit
        0x13A6 => {
            println!("https://prizm.cemetech.net/index.php?title=App_LINK_TransmitInit");
        }
        // https://prizm.cemetech.net/index.php?title=APP_LINK_transmit_select_dialog
        0x1384 => {
            println!("https://prizm.cemetech.net/index.php?title=APP_LINK_transmit_select_dialog");
        }
        // https://prizm.cemetech.net/index.php?title=APP_MEMORY
        0x1632 => {
            println!("https://prizm.cemetech.net/index.php?title=APP_MEMORY");
        }
        // https://prizm.cemetech.net/index.php?title=App_Optimize
        0x1630 => {
            println!("https://prizm.cemetech.net/index.php?title=App_Optimize");
        }
        // https://prizm.cemetech.net/index.php?title=APP_Program
        0x1945 => {
            println!("https://prizm.cemetech.net/index.php?title=APP_Program");
        }
        // https://prizm.cemetech.net/index.php?title=APP_RUNMAT
        0x1A03 => {
            println!("https://prizm.cemetech.net/index.php?title=APP_RUNMAT");
        }
        // https://prizm.cemetech.net/index.php?title=APP_SYSTEM
        0x1E13 => {
            println!("https://prizm.cemetech.net/index.php?title=APP_SYSTEM");
        }
        // https://prizm.cemetech.net/index.php?title=APP_SYSTEM_BATTERY
        0x1E07 => {
            println!("https://prizm.cemetech.net/index.php?title=APP_SYSTEM_BATTERY");
        }
        // https://prizm.cemetech.net/index.php?title=APP_SYSTEM_DISPLAY
        0x1E0A => {
            println!("https://prizm.cemetech.net/index.php?title=APP_SYSTEM_DISPLAY");
        }
        // https://prizm.cemetech.net/index.php?title=APP_SYSTEM_LANGUAGE
        0x1E0D => {
            println!("https://prizm.cemetech.net/index.php?title=APP_SYSTEM_LANGUAGE");
        }
        // https://prizm.cemetech.net/index.php?title=APP_SYSTEM_POWER
        0x1E05 => {
            println!("https://prizm.cemetech.net/index.php?title=APP_SYSTEM_POWER");
        }
        // https://prizm.cemetech.net/index.php?title=APP_SYSTEM_RESET
        0x1E17 => {
            println!("https://prizm.cemetech.net/index.php?title=APP_SYSTEM_RESET");
        }
        // https://prizm.cemetech.net/index.php?title=APP_SYSTEM_VERSION
        0x1E34 => {
            println!("https://prizm.cemetech.net/index.php?title=APP_SYSTEM_VERSION");
        }
        _ => unimplemented!("Unknown Syscall"),
    }
}
