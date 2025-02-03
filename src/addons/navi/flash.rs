// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn nvflash_detect() -> c_int;
    pub fn nvflash_erase_block(addr: u32) -> c_int;
    pub fn nvflash_write_block(addr: u32, data: *mut c_void, len: u32) -> c_int;
    pub fn nvflash_erase_all() -> c_int;
}
