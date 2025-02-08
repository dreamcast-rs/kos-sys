// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

use crate::os::blockdev::kos_blockdev_t;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn sd_crc7(data: *const u8, size: c_int, crc: u8) -> u8;
    pub fn sd_init() -> c_int;
    pub fn sd_shutdown() -> c_int;
    pub fn sd_read_blocks(block: u32, count: c_size_t, buf: *mut u8) -> c_int;
    pub fn sd_write_blocks(block: u32, count: c_size_t, buf: *const u8) -> c_int;
    pub fn sd_get_size() -> u64;
    pub fn sd_blockdev_for_partition(
        partition: c_int,
        rv: *mut kos_blockdev_t,
        partition_type: *mut u8,
    ) -> c_int;
}
