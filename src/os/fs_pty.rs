// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

use crate::os::fs::file_t;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn fs_pty_create(
        buffer: *mut c_char,
        maxbuflen: c_int,
        master_out: *mut file_t,
        slave_out: *mut file_t,
    ) -> c_int;
    pub fn fs_pty_init();
    pub fn fs_pty_shutdown();
}
