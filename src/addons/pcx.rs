// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn pcx_load_flat(
        r#fn: *const c_char,
        w_out: *mut c_int,
        h_out: *mut c_int,
        pic_out: *mut c_void,
    ) -> c_int;
    pub fn pcx_load_palette(
        r#fn: *const c_char,
        w_out: *mut c_int,
        h_out: *mut c_int,
        pic_out: *mut c_void,
        pal_out: *mut c_void,
    ) -> c_int;
}
