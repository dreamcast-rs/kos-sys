// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[link(name = "kallisti")]
extern "C" {
    pub fn fs_vmu_init() -> c_int;
    pub fn fs_vmu_shutdown() -> c_int;
}
