// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn fs_ramdisk_init() -> c_int;
    pub fn fs_ramdisk_shutdown() -> c_int;
    pub fn fs_ramdisk_attach(r#fn: *const c_char, obj: *mut c_void,
                             size: c_size_t) -> c_int;
    pub fn fs_ramdisk_detach(r#fn: *const c_char, obj: *mut *mut c_void,
                             size: *mut c_size_t) -> c_int;
}
