// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub const DT_UNKNOWN: u8    = 0;
pub const DT_FIFO: u8       = 1;
pub const DT_CHR: u8        = 2;
pub const DT_DIR: u8        = 4;
pub const DT_BLK: u8        = 6;
pub const DT_REG: u8        = 8;
pub const DT_LNK: u8        = 10;
pub const DT_SOCK: u8       = 12;
pub const DT_WHT: u8        = 14;

#[repr(C)]
pub struct dirent {
    pub d_ino:      c_int,
    pub d_off:      off_t,
    pub d_reclen:   u16,
    pub d_type:     u8,
    pub d_name:     FAM<c_char>,
}

// FIXME: Define DIR

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn opendir(name: *const c_char) -> *mut DIR;
    pub fn closedir(dir: *mut DIR) -> c_int;
    pub fn readdir(dir: *mut DIR) -> *mut dirent;
    pub fn dirfd(dirp: *mut DIR) -> c_int;
    pub fn rewinddir(dir: *mut DIR);
    pub fn scandir(
        dir: *const c_char,
        namelist: *mut *mut *mut dirent,
        filter: Option<unsafe extern "C" fn(*const dirent) -> c_int>,
        compar: Option<unsafe extern "C" fn(*mut *const dirent,
                                            *mut *const dirent) -> c_int>
    ) -> c_int;
    pub fn alphasort(a: *mut *const dirent, b: *mut *const dirent) -> c_int;
    pub fn seekdir(dir: *mut DIR, offset: off_t);
    pub fn telldir(dir: *mut DIR) -> off_t;
}
