// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

use super::thread::kthread_t;

#[repr(C)]
pub struct mutex_t {
    pub r#type:     c_int,
    pub dynamic:    c_int,
    pub holder:     *mut kthread_t,
    pub count:      c_int,
}

pub const MUTEX_TYPE_NORMAL: c_int      = 0;
pub const MUTEX_TYPE_OLDNORMAL: c_int   = 1;
pub const MUTEX_TYPE_ERRORCHECK: c_int  = 2;
pub const MUTEX_TYPE_RECURSIVE: c_int   = 3;

pub const MUTEX_TYPE_DEFAULT: c_int     = MUTEX_TYPE_NORMAL;

pub const MUTEX_INITIALIZER: mutex_t = mutex_t {
                                           r#type: MUTEX_TYPE_NORMAL,
                                           dynamic: 0,
                                           holder: core::ptr::null_mut(),
                                           count: 0,
                                       };

pub const ERRORCHECK_MUTEX_INITIALIZER: mutex_t = mutex_t {
                                                      r#type: MUTEX_TYPE_ERRORCHECK,
                                                      dynamic: 0,
                                                      holder: core::ptr::null_mut(),
                                                      count: 0,
                                                  };

pub const RECURSIVE_MUTEX_INITIALIZER: mutex_t = mutex_t {
                                                     r#type: MUTEX_TYPE_RECURSIVE,
                                                     dynamic: 0,
                                                     holder: core::ptr::null_mut(),
                                                     count: 0,
                                                 };

// FIXME: Implement scoped mutex using RAII/Drop

#[link(name = "kallisti")]
extern "C" {
    pub fn mutex_init(m: *mut mutex_t, mtype: c_int) -> c_int;
    pub fn mutex_destroy(m: *mut mutex_t) -> c_int;
    pub fn mutex_lock(m: *mut mutex_t) -> c_int;
    pub fn mutex_lock_irqsafe(m: *mut mutex_t) -> c_int;
    pub fn mutex_lock_timed(m: *mut mutex_t, timeout: c_int) -> c_int;
    pub fn mutex_is_locked(m: *mut mutex_t) -> c_int;
    pub fn mutex_trylock(m: *mut mutex_t) -> c_int;
    pub fn mutex_unlock(m: *mut mutex_t) -> c_int;
    pub fn mutex_unlock_as_thread(m: *mut mutex_t, thd: *mut kthread_t) -> c_int;
}
