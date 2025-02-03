// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub type kthread_key_t = c_int;

#[repr(C)]
pub struct kv_list {
    pub le_next:    *mut kthread_tls_kv_t,
    pub le_prev:    *mut *mut kthread_tls_kv_t,
}

#[repr(C)]
pub struct kthread_tls_kv_t {
    pub kv_list:    kv_list,
    key:            kthread_key_t,
    data:           *mut c_void,
    destructor:     Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
pub struct kthread_tls_kv_list {
    pub lh_first:   *mut kthread_tls_kv_t,
}

#[link(name = "kallisti")]
extern "C" {
    pub fn kthread_key_next() -> kthread_key_t;
    pub fn kthread_key_create(key: *mut kthread_key_t,
                              destructor: Option<unsafe extern "C" fn(*mut c_void)>)
                              -> c_int;
    pub fn kthread_getspecific(key: kthread_key_t) -> *mut c_void;
    pub fn kthread_setspecific(key: kthread_key_t, value: *const c_void) -> c_int;
    pub fn kthread_key_delete(key: kthread_key_t) -> c_int;
    pub fn kthread_key_delete_destructor(key: kthread_key_t);
    pub fn kthread_tls_init() -> c_int;
    pub fn kthread_tls_shutdown();
}
