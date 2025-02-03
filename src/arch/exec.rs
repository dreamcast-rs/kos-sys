// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn arch_exec_at(image: *const c_void, length: u32, address: u32) -> !;
    pub fn arch_exec(image: *const c_void, length: u32) -> !;
}
