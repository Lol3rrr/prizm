pub fn is_syscall(id: u32) -> bool {
    match id {
        0x1F42 | 0x1F44 | 0x1F46 => true,
        _ => false,
    }
}

pub fn handle_syscall(id: u32) {
    match id {
        // https://prizm.cemetech.net/index.php?title=Sys_free
        0x1F42 => {
            println!("https://prizm.cemetech.net/index.php?title=Sys_free");
        }
        // https://prizm.cemetech.net/index.php?title=Sys_malloc
        0x1F44 => {
            println!("https://prizm.cemetech.net/index.php?title=Sys_malloc");
        }
        // https://prizm.cemetech.net/index.php?title=Sys_realloc
        0x1F46 => {
            println!("https://prizm.cemetech.net/index.php?title=Sys_realloc");
        }
        _ => unimplemented!("Unknown Syscall"),
    }
}
