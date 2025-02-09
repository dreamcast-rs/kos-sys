// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub const SCHED_OTHER: c_int    = 0;
pub const SCHED_FIFO: c_int     = 1;
pub const SCHED_RR: c_int       = 2;

#[repr(C)]
pub struct sched_param {
    sched_priority: c_int,
}
