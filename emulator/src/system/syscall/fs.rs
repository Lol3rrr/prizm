const BFILE_CLOSEFILE_OS: u32 = 0x1da4;
const BFILE_CREATEENTRY_OS: u32 = 0x1dae;
const BFILE_DELETEENTRY: u32 = 0x1db4;

pub fn is_syscall(id: u32) -> bool {
    match id {
        BFILE_CLOSEFILE_OS | BFILE_CREATEENTRY_OS | BFILE_DELETEENTRY | 0x1dba | 0x1db7
        | 0x1db6 | 0x1db9 | 0x1db8 | 0x1da6 | 0x1da5 | 0x1ddb | 0x1dda | 0x1da3 | 0x1dac
        | 0x1db3 | 0x1da9 | 0x1ddc | 0x1dab | 0x1daf => true,
        _ => false,
    }
}

pub fn handle_syscall(id: u32, param_1: u32, param_2: u32, param_3: u32, param_4: u32) {
    match id {
        // https://prizm.cemetech.net/index.php?title=Bfile_CloseFile_OS
        BFILE_CLOSEFILE_OS => {
            println!("BFile_CloseFile_OS");
            println!("Handle: {}", param_1);
        }
        // https://prizm.cemetech.net/index.php?title=Bfile_CreateEntry_OS
        BFILE_CREATEENTRY_OS => {
            println!("BFile_CreateEntry_OS");
            println!("Filename: {}", param_1);
            println!("Mode: {}", param_2);
            println!("Size: {}", param_3);
        }
        // https://prizm.cemetech.net/index.php?title=Bfile_DeleteEntry
        BFILE_DELETEENTRY => {
            println!("Bfile_DeleteEntry");
            println!("Filename: {}", param_1);
        }
        // https://prizm.cemetech.net/index.php?title=Bfile_FindClose
        0x1dba => {
            println!("Bfile_FindClose");
            println!("FindHandle: {}", param_1);
        }
        // https://prizm.cemetech.net/index.php?title=Bfile_FindFirst
        0x1db7 => {
            println!("Bfile_FindFirst");
            println!("PathName: {}", param_1);
            println!("FindHandle: {}", param_2);
            println!("FoundFile: {}", param_3);
            println!("FileInfo: {}", param_4);
        }
        // https://prizm.cemetech.net/index.php?title=Bfile_FindFirst_NON_SMEM
        0x1db6 => {
            println!("Bfile_FindFirst_NON_SMEM");
            println!("PathName: {}", param_1);
            println!("FindHandle: {}", param_2);
            println!("FoundFile: {}", param_3);
            println!("FileInfo: {}", param_4);
        }
        // https://prizm.cemetech.net/index.php?title=Bfile_FindNext
        0x1db9 => {
            println!("Bfile_FindNext");
            println!("FindHandle: {}", param_1);
            println!("FoundFile: {}", param_2);
            println!("FileInfo: {}", param_3);
        }
        // https://prizm.cemetech.net/index.php?title=Bfile_FindNext_NON_SMEM
        0x1db8 => {
            println!("Bfile_FindNext_NON_SMEM");
            println!("FindHandle: {}", param_1);
            println!("FoundFile: {}", param_2);
            println!("FileInfo: {}", param_3);
        }
        // https://prizm.cemetech.net/index.php?title=Bfile_GetFileSize_OS
        0x1da6 => {
            println!("Bfile_GetFileSize_OS");
            println!("Handle: {}", param_1);
        }
        // https://prizm.cemetech.net/index.php?title=Bfile_GetMediaFree_OS
        0x1da5 => {
            println!("Bfile_GetMediaFree_OS");
            println!("Media-ID: {}", param_1);
            println!("Freespace: {}", param_2);
        }
        // https://prizm.cemetech.net/index.php?title=Bfile_NameToStr_ncpy
        0x1ddb => {
            println!("Bfile_NameToStr_ncpy");
            println!("Destination: {}", param_1);
            println!("Source: {}", param_2);
            println!("n: {}", param_3);
        }
        // https://prizm.cemetech.net/index.php?title=Bfile_Name_MatchMask
        0x1dda => {
            println!("Bfile_Name_MatchMask");
            println!("Mask: {}", param_1);
            println!("FileName: {}", param_2);
        }
        // https://prizm.cemetech.net/index.php?title=Bfile_OpenFile_OS
        0x1da3 => {
            println!("Bfile_OpenFile_OS");
            println!("FileName: {}", param_1);
            println!("Mode: {}", param_2);
            println!("Zero: {}", param_3);
        }
        // https://prizm.cemetech.net/index.php?title=Bfile_ReadFile_OS
        0x1dac => {
            println!("Bfile_ReadFile_OS");
            println!("Handle: {}", param_1);
            println!("Buf: {}", param_2);
            println!("Size: {}", param_3);
            println!("Pos: {}", param_4);
        }
        // https://prizm.cemetech.net/index.php?title=Bfile_RenameEntry
        0x1db3 => {
            println!("Bfile_RenameEntry");
            println!("Old-Path: {}", param_1);
            println!("New-Path: {}", param_2);
        }
        // https://prizm.cemetech.net/index.php?title=Bfile_SeekFile_OS
        0x1da9 => {
            println!("Bfile_SeekFile_OS");
            println!("Handle: {}", param_1);
            println!("Pos: {}", param_2);
        }
        // https://prizm.cemetech.net/index.php?title=Bfile_StrToName_ncpy
        0x1ddc => {
            println!("Bfile_StrToName_ncpy");
            println!("Dest: {}", param_1);
            println!("Source: {}", param_2);
            println!("n: {}", param_3);
        }
        // https://prizm.cemetech.net/index.php?title=Bfile_TellFile_OS
        0x1dab => {
            println!("Bfile_TellFile_OS");
            println!("Handle: {}", param_1);
        }
        // https://prizm.cemetech.net/index.php?title=Bfile_TellFile_OS
        0x1daf => {
            println!("Bfile_WriteFile_OS");
            println!("Handle: {}", param_1);
            println!("Buf: {}", param_2);
            println!("Size: {}", param_3);
        }
        _ => panic!("Unexpected Filesystem-Syscall"),
    }
}
