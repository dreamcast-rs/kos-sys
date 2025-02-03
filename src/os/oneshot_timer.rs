// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[repr(C)]
pub struct oneshot_timer_t {
    _opaque:    [u8; 0],
}

#[inline]
pub extern "C" fn oneshot_timer_reset(timer: *mut oneshot_timer_t) {
    unsafe {
        oneshot_timer_stop(timer);
        oneshot_timer_start(timer);
    }
}

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn oneshot_timer_create(cb: Option<unsafe extern "C" fn(*mut c_void)>,
                                data: *mut c_void, timeout_ms: c_uint)
                                -> *mut oneshot_timer_t;
    pub fn oneshot_timer_destroy(timer: *mut oneshot_timer_t);
    pub fn oneshot_timer_setup(timer: *mut oneshot_timer_t, cb:
                               Option<unsafe extern "C" fn(*mut c_void)>,
                               data: *mut c_void, timeout_ms: c_uint);
    pub fn oneshot_timer_start(timer: *mut oneshot_timer_t);
    pub fn oneshot_timer_stop(timer: *mut oneshot_timer_t);
}
