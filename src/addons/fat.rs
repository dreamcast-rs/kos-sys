// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub const FS_FAT_MOUNT_READONLY: u32    = 0x00000000;
pub const FS_FAT_MOUNT_READWRITE: u32   = 0x00000001;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn fs_fat_init() -> c_int;
    pub fn fs_fat_shutdown() -> c_int;
    pub fn fs_fat_mount(mp: *const c_char, dev: *mut crate::os::blockdev::kos_blockdev_t,
                        flags: u32) -> c_int;
    pub fn fs_fat_unmount(mp: *const c_char) -> c_int;
    pub fn fs_fat_sync(mp: *const c_char) -> c_int;
}
