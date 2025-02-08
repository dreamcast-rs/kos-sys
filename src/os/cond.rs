// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

use super::mutex::mutex_t;

#[repr(C)]
pub struct condvar_t {
    pub dummy:      c_int,
    pub dynamic:    c_int,
}

pub const COND_INITIALIZER: condvar_t   = condvar_t {
    dummy: 0,
    dynamic: 0,
};

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn cond_init(cv: *mut condvar_t) -> c_int;
    pub fn cond_destroy(cv: *mut condvar_t) -> c_int;
    pub fn cond_wait(cv: *mut condvar_t, m: *mut mutex_t) -> c_int;
    pub fn cond_wait_timed(cv: *mut condvar_t, m: *mut mutex_t, timeout: c_int) -> c_int;
    pub fn cond_signal(cv: *mut condvar_t) -> c_int;
    pub fn cond_broadcast(cv: *mut condvar_t) -> c_int;
}
