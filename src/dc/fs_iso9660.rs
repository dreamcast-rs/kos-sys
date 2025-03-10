// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn iso_reset() -> c_int;
    pub fn fs_iso9660_init();
    pub fn fs_iso9660_shutdown();
}
