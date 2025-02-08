// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

use crate::os::cond::condvar_t;
use crate::os::mutex::mutex_t;
use crate::os::once::KTHREAD_ONCE_INIT;
use crate::os::once::kthread_once_t;
use crate::os::thread::kthread_t;
use crate::os::tls::kthread_key_t;

pub const thrd_success: c_int           = 0;
pub const thrd_error: c_int             = -1;
pub const thrd_timedout: c_int          = -2;
pub const thrd_busy: c_int              = -3;
pub const thrd_nomem: c_int             = -4;

pub type once_flag = kthread_once_t;

pub const ONCE_FLAG_INIT: once_flag = KTHREAD_ONCE_INIT;

pub type mtx_t = mutex_t;

pub const mtx_plain: c_int              = 1 << 0;
pub const mtx_recursive: c_int          = 1 << 1;
pub const mtx_timed: c_int              = 1 << 2;

pub type cnd_t = condvar_t;

pub type thrd_t = *mut kthread_t;

pub type thrd_start_t = Option<unsafe extern "C" fn(*mut c_void) -> c_int>;

pub const TSS_DTOR_ITERATIONS: usize    = 1;

pub type tss_t = kthread_key_t;

pub type tss_dtor_t = Option<unsafe extern "C" fn(*mut c_void)>;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn call_once(flag: *mut once_flag, func: Option<unsafe extern "C" fn()>);
    pub fn mtx_destroy(mtx: *mut mtx_t);
    pub fn mtx_init(mtx: *mut mtx_t, r#type: c_int) -> c_int;
    pub fn mtx_lock(mtx: *mut mtx_t) -> c_int;
    pub fn mtx_timedlock(mtx: *mut mtx_t, ts: *const timespec) -> c_int;
    pub fn mtx_trylock(mtx: *mut mtx_t) -> c_int;
    pub fn mtx_unlock(mtx: *mut mtx_t) -> c_int;
    pub fn cnd_broadcast(cond: *mut cnd_t) -> c_int;
    pub fn cnd_destroy(cond: *mut cnd_t);
    pub fn cnd_init(cond: *mut cnd_t) -> c_int;
    pub fn cnd_signal(cond: *mut cnd_t) -> c_int;
    pub fn cnd_timedwait(cond: *mut cnd_t, mtx: *mut mtx_t, ts: *const timespec) -> c_int;
    pub fn cnd_wait(cond: *mut cnd_t, mtx: *mut mtx_t) -> c_int;
    pub fn thrd_create(thr: *mut thrd_t, func: thrd_start_t, arg: *mut c_void) -> c_int;
    pub fn thrd_current() -> thrd_t;
    pub fn thrd_detach(thr: thrd_t) -> c_int;
    pub fn thrd_equal(thr0: thrd_t, thr1: thrd_t) -> c_int;
    pub fn thrd_exit(res: c_int) -> !;
    pub fn thrd_join(thr: thrd_t, res: *mut c_int) -> c_int;
    pub fn thrd_sleep(duration: *const timespec, remaining: *mut timespec) -> c_int;
    pub fn thrd_yield();
    pub fn tss_create(key: *mut tss_t, dtor: tss_dtor_t) -> c_int;
    pub fn tss_delete(key: tss_t);
    pub fn tss_get(key: tss_t) -> *mut c_void;
    pub fn tss_set(key: tss_t, val: *mut c_void) -> c_int;
}
