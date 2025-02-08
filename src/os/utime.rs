// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[repr(C)]
pub struct utimbuf {
    pub actime:     time_t,
    pub modtime:    time_t,
}

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn utime(path: *const c_char, times: *mut utimbuf) -> c_int;
}
