// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn fs_romdisk_init();
    pub fn fs_romdisk_shutdown();
    pub fn fs_romdisk_mount(mountpoint: *const c_char, img: *const u8,
                            own_buffer: c_int) -> c_int;
    pub fn fs_romdisk_unmount(mountpoint: *const c_char) -> c_int;
}
