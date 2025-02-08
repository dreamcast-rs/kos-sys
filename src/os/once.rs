// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub type kthread_once_t = c_int; // volatile

pub const KTHREAD_ONCE_INIT: kthread_once_t  = 0;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn kthread_once(once_control: *mut kthread_once_t,
                        init_routine: Option<unsafe extern "C" fn()>) -> c_int;
}
