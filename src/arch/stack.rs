// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[link(name = "kallisti")]
extern "C" {
    pub fn arch_stk_trace(n: c_int);
    pub fn arch_stk_trace_at(fp: u32, n: c_size_t);
}
