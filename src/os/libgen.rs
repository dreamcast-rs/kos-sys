// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn basename(path: *mut c_char) -> *mut c_char;
    pub fn dirname(path: *mut c_char) -> *mut c_char;
}
