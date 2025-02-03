// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub const _UTSNAME_LENGTH: usize    = 64;

#[repr(C)]
pub struct utsname {
    sysname:    [c_char; _UTSNAME_LENGTH],
    nodename:   [c_char; _UTSNAME_LENGTH],
    release:    [c_char; _UTSNAME_LENGTH],
    version:    [c_char; _UTSNAME_LENGTH],
    machine:    [c_char; _UTSNAME_LENGTH],
}

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn uname(n: *mut utsname);
}
