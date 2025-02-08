// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

use super::thread::kthread_t;

unsafe extern "C" {
    pub fn genwait_wait(
        obj: *mut c_void,
        mesg: *const c_char,
        timeout: c_int,
        callback: Option<unsafe extern "C" fn(*mut c_void)>,
    ) -> c_int;
    pub fn genwait_wake_cnt(obj: *mut c_void, cnt: c_int, err: c_int) -> c_int;
    pub fn genwait_wake_all(obj: *mut c_void);
    pub fn genwait_wake_one(obj: *mut c_void);
    pub fn genwait_wake_all_err(obj: *mut c_void, err: c_int);
    pub fn genwait_wake_one_err(obj: *mut c_void, err: c_int);
    pub fn genwait_wake_thd(obj: *mut c_void, thd: *mut kthread_t, err: c_int) -> c_int;
    pub fn genwait_check_timeouts(now: u64);
    pub fn genwait_next_timeout() -> u64;
    pub fn genwait_init() -> c_int;
    pub fn genwait_shutdown();
}
