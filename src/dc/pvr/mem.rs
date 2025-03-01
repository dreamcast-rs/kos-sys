// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub type pvr_ptr_t = *mut c_void;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn pvr_mem_malloc(size: c_size_t) -> pvr_ptr_t;
    pub fn pvr_mem_free(chunk: pvr_ptr_t);
    pub fn pvr_mem_available() -> c_size_t;
    pub fn pvr_mem_reset();
    pub fn pvr_mem_print_list();
    pub fn pvr_mem_stats();
}
