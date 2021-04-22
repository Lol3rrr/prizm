pub fn is_syscall(id: u32) -> bool {
    match id {
        0x02C2 | 0x2C1 | 0x02C0 | 0x02BF | 0x11DE => true,
        _ => false,
    }
}

pub fn handle_syscall(id: u32) {
    match id {
        // https://prizm.cemetech.net/index.php?title=RTC_Elapsed_ms
        0x02C2 => {
            println!("https://prizm.cemetech.net/index.php?title=RTC_Elapsed_ms");
        }
        // https://prizm.cemetech.net/index.php?title=RTC_GetTicks
        0x2C1 => {
            println!("https://prizm.cemetech.net/index.php?title=RTC_GetTicks");
        }
        // https://prizm.cemetech.net/index.php?title=RTC_GetTime
        0x02C0 => {
            println!("https://prizm.cemetech.net/index.php?title=RTC_GetTime");
        }
        // https://prizm.cemetech.net/index.php?title=RTC_Reset
        0x02BF => {
            println!("https://prizm.cemetech.net/index.php?title=RTC_Reset");
        }
        // https://prizm.cemetech.net/index.php?title=RTC_SetDateTime
        0x11DE => {
            println!("https://prizm.cemetech.net/index.php?title=RTC_SetDateTime");
        }
        _ => unimplemented!("Unknown Syscall"),
    }
}
