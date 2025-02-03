// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

use super::thread::{kthread_attr_t, kthread_t};

#[repr(C)]
pub struct kthread_worker_t {
    _opaque:        [u8; 0],
}

#[repr(C)]
pub struct entry {
    pub stqe_next:  *mut kthread_job_t,
}

#[repr(C)]
pub struct kthread_job_t {
    pub entry:      entry,
    pub data:       *mut c_void,
}

#[inline]
pub unsafe fn thd_worker_create(routine: Option<unsafe extern "C" fn(*mut c_void)>,
                                data: *mut c_void) -> *mut kthread_worker_t {
    unsafe {
        thd_worker_create_ex(null(), routine, data)
    }
}

#[link(name = "kallisti")]
extern "C" {
    pub fn thd_worker_create_ex(attr: *const kthread_attr_t,
                                routine: Option<unsafe extern "C" fn(*mut c_void)>,
                                data: *mut c_void) -> *mut kthread_worker_t;
    pub fn thd_worker_destroy(thd: *mut kthread_worker_t);
    pub fn thd_worker_wakeup(thd: *mut kthread_worker_t);
    pub fn thd_worker_get_thread(thd: *mut kthread_worker_t) -> *mut kthread_t;
    pub fn thd_worker_add_job(thd: *mut kthread_worker_t, job: *mut kthread_job_t);
    pub fn thd_worker_dequeue_job(worker: *mut kthread_worker_t) -> *mut kthread_job_t;
}
