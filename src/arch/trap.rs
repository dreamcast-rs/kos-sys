// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;
use super::irq::irq_context_t;

pub type trapa_t = u8;

pub type trapa_handler = Option<unsafe extern "C" fn(trap: trapa_t,
                                                     context: *mut irq_context_t,
                                                     data: *mut c_void)>;

#[link(name = "kallisti")]
extern "C" {
    pub fn trapa_set_handler(trap: trapa_t, hnd: trapa_handler,
                             data: *mut c_void) -> c_int;
    pub fn trapa_get_handler(trap: trapa_t, data: *mut *mut c_void) -> trapa_handler;
}
