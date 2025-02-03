// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub static dbgio_fb: crate::os::dbgio::dbgio_handler_t;
    pub fn dbgio_fb_set_target(t: *mut u16, w: c_int, h: c_int,
                               borderx: c_int, bordery: c_int);
}
