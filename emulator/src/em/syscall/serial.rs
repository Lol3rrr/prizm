pub fn is_syscall(id: u32) -> bool {
    match id {
        0x1BBC | 0x1BC1 | 0x1BC2 | 0x1BBD | 0x1BC0 | 0x1BBF | 0x1BBA | 0x1BB9 | 0x1BBB | 0x1BC1
        | 0x1BC2 | 0x1BB8 | 0x1BC6 | 0x1BB7 | 0x1BBB | 0x1BBF | 0x1BC0 | 0x1BBA | 0x1BB9
        | 0x1BBE | 0x1BBC | 0x1BBD => true,
        _ => false,
    }
}

pub fn handle_syscall(id: u32) {
    match id {
        // https://prizm.cemetech.net/index.php?title=Serial_BufferedTransmitOneByte
        0x1BBC => {
            println!("https://prizm.cemetech.net/index.php?title=Serial_BufferedTransmitOneByte");
        }
        // https://prizm.cemetech.net/index.php?title=Serial_ClearReceiveBuffer
        0x1BC1 => {
            println!("https://prizm.cemetech.net/index.php?title=Serial_ClearReceiveBuffer");
        }
        // https://prizm.cemetech.net/index.php?title=Serial_ClearTransmitBuffer
        0x1BC2 => {
            println!("https://prizm.cemetech.net/index.php?title=Serial_ClearTransmitBuffer");
        }
        // https://prizm.cemetech.net/index.php?title=Serial_DirectTransmitOneByte
        0x1BBD => {
            println!("https://prizm.cemetech.net/index.php?title=Serial_DirectTransmitOneByte");
        }
        // https://prizm.cemetech.net/index.php?title=Serial_GetFreeTransmitSpace
        0x1BC0 => {
            println!("https://prizm.cemetech.net/index.php?title=Serial_GetFreeTransmitSpace");
        }
        // https://prizm.cemetech.net/index.php?title=Serial_GetReceivedBytesAvailable
        0x1BBF => {
            println!("https://prizm.cemetech.net/index.php?title=Serial_GetReceivedBytesAvailable");
        }
        // https://prizm.cemetech.net/index.php?title=Serial_ReadNBytes
        0x1BBA => {
            println!("https://prizm.cemetech.net/index.php?title=Serial_ReadNBytes");
        }
        // https://prizm.cemetech.net/index.php?title=Serial_ReadOneByte
        0x1BB9 => {
            println!("https://prizm.cemetech.net/index.php?title=Serial_ReadOneByte");
        }
        // https://prizm.cemetech.net/index.php?title=Serial_SpyNthByte
        0x1BBB => {
            println!("https://prizm.cemetech.net/index.php?title=Serial_SpyNthByte");
        }
        // https://prizm.cemetech.net/index.php?title=Serial_ClearRX
        0x1BC1 => {
            println!("https://prizm.cemetech.net/index.php?title=Serial_ClearRX");
        }
        // https://prizm.cemetech.net/index.php?title=Serial_ClearTX
        0x1BC2 => {
            println!("https://prizm.cemetech.net/index.php?title=Serial_ClearTX");
        }
        // https://prizm.cemetech.net/index.php?title=Serial_Close
        0x1BB8 => {
            println!("https://prizm.cemetech.net/index.php?title=Serial_Close");
        }
        // https://prizm.cemetech.net/index.php?title=Serial_IsOpen
        0x1BC6 => {
            println!("https://prizm.cemetech.net/index.php?title=Serial_IsOpen");
        }
        // https://prizm.cemetech.net/index.php?title=Serial_Open
        0x1BB7 => {
            println!("https://prizm.cemetech.net/index.php?title=Serial_Open");
        }
        // https://prizm.cemetech.net/index.php?title=Serial_Peek
        0x1BBB => {
            println!("https://prizm.cemetech.net/index.php?title=Serial_Peek");
        }
        // https://prizm.cemetech.net/index.php?title=Serial_PollRX
        0x1BBF => {
            println!("https://prizm.cemetech.net/index.php?title=Serial_PollRX");
        }
        // https://prizm.cemetech.net/index.php?title=Serial_PollTX
        0x1BC0 => {
            println!("https://prizm.cemetech.net/index.php?title=Serial_PollTX");
        }
        // https://prizm.cemetech.net/index.php?title=Serial_Read
        0x1BBA => {
            println!("https://prizm.cemetech.net/index.php?title=Serial_Read");
        }
        // https://prizm.cemetech.net/index.php?title=Serial_ReadSingle
        0x1BB9 => {
            println!("https://prizm.cemetech.net/index.php?title=Serial_ReadSingle");
        }
        // https://prizm.cemetech.net/index.php?title=Serial_Write
        0x1BBE => {
            println!("https://prizm.cemetech.net/index.php?title=Serial_Write");
        }
        // https://prizm.cemetech.net/index.php?title=Serial_WriteSingle
        0x1BBC => {
            println!("https://prizm.cemetech.net/index.php?title=Serial_WriteSingle");
        }
        // https://prizm.cemetech.net/index.php?title=Serial_WriteUnbuffered
        0x1BBD => {
            println!("https://prizm.cemetech.net/index.php?title=Serial_WriteUnbuffered");
        }
        _ => unimplemented!("Unknown Syscall"),
    }
}
