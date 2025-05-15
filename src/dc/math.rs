// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn bit_reverse(value: c_uint) -> c_uint;
    pub fn bit_reverse8(value: c_uchar) -> c_uchar;
}
