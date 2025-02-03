// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[link(name = "kallisti")]
extern "C" {
    pub fn fs_null_init() -> c_int;
    pub fn fs_null_shutdown() -> c_int;
}
