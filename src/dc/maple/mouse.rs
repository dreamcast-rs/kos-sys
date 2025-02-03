// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub const MOUSE_RIGHTBUTTON: u16    = 1 << 1;
pub const MOUSE_LEFTBUTTON: u16     = 1 << 2;
pub const MOUSE_SIDEBUTTON: u16     = 1 << 3;

pub const MOUSE_DELTA_CENTER: u16   = 0x200;

#[repr(C)]
pub struct mouse_cond_t {
    pub buttons:    u16,
    pub dummy1:     u16,
    pub dx:         i16,
    pub dy:         i16,
    pub dz:         i16,
    pub dummy2:     u16,
    pub dummy3:     u32,
    pub dummy4:     u32,
}

#[repr(C)]
pub struct mouse_state_t {
    pub buttons:    u32,
    pub dx:         c_int,
    pub dy:         c_int,
    pub dz:         c_int,
}

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn mouse_init();
    pub fn mouse_shutdown();
}
