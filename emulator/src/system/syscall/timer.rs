use crate::traits::Debugger;

pub fn is_syscall(id: u32) -> bool {
    match id {
        0x08DB | 0x08D9 | 0x08DA | 0x08DC => true,
        _ => false,
    }
}

pub fn handle_syscall(id: u32, debug: &dyn Debugger) {
    match id {
        // https://prizm.cemetech.net/index.php?title=Timer_Deinstall
        0x08DA => {
            debug.print("https://prizm.cemetech.net/index.php?title=Timer_Deinstall");
        }
        // https://prizm.cemetech.net/index.php?title=Timer_Install
        0x08D9 => {
            debug.print("https://prizm.cemetech.net/index.php?title=Timer_Install");
        }
        // https://prizm.cemetech.net/index.php?title=Timer_Start
        0x08DB => {
            debug.print("https://prizm.cemetech.net/index.php?title=Timer_Start");
        }
        // https://prizm.cemetech.net/index.php?title=Timer_Stop
        0x08DC => {
            debug.print("https://prizm.cemetech.net/index.php?title=Timer_Stop");
        }
        _ => unimplemented!("Unknown Syscall"),
    }
}
