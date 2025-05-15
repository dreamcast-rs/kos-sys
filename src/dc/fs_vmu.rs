// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;
use crate::os::fs::file_t;
use crate::dc::vmu_pkg::vmu_pkg_t;

pub const IOCTL_VMU_SET_HDR: c_int  = 0x564d5530;

#[link(name = "kallisti")]
unsafe extern "C" {
    #[link_name = "fs_vmu_set_header_wrapper"]
    pub fn fs_vmu_set_header(fd: file_t, pkg: *const vmu_pkg_t) -> c_int;
    #[link_name = "fs_vmu_set_default_header_wrapper"]
    pub fn fs_vmu_set_default_header(pkg: *const vmu_pkg_t) -> c_int;
    pub fn fs_vmu_init() -> c_int;
    pub fn fs_vmu_shutdown() -> c_int;
}
