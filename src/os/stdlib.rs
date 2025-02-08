// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn posix_memalign(memptr: *mut *mut c_void, alignment: c_size_t, size: c_size_t) -> c_int;
}
