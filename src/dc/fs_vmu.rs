// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn fs_vmu_init() -> c_int;
    pub fn fs_vmu_shutdown() -> c_int;
}
