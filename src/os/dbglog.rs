// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub const DBG_DEAD: c_int       = 0;
pub const DBG_CRITICAL: c_int   = 1;
pub const DBG_ERROR: c_int      = 2;
pub const DBG_WARNING: c_int    = 3;
pub const DBG_NOTICE: c_int     = 4;
pub const DBG_INFO: c_int       = 5;
pub const DBG_DEBUG: c_int      = 6;
pub const DBG_KDEBUG: c_int     = 7;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn dbglog(level: c_int, fmt: *const c_char, ...);
    pub fn dbglog_set_level(level: c_int);
}
