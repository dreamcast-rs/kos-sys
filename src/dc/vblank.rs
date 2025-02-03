// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn vblank_handler_add(hnd: super::asic::asic_evt_handler,
                              data: *mut c_void) -> c_int;
    pub fn vblank_handler_remove(handle: c_int) -> c_int;
    pub fn vblank_init() -> c_int;
    pub fn vblank_shutdown() -> c_int;
}
