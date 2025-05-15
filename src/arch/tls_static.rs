// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::os::thread::kthread_t;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn arch_tls_init();
    pub fn arch_tls_setup_data(thd: *mut kthread_t);
    pub fn arch_tls_destroy_data(thd: *mut kthread_t);
}
