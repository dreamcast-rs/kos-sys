// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[repr(C)]
pub struct semaphore_t {
    pub initialized:    c_int,
    pub count:          c_int,
}

#[macro_export]
macro_rules! SEM_INITIALIZER {
    ($value:expr) => {
        semaphore_t { initialized: 1, count: $value }
    };
}

#[link(name = "kallisti")]
extern "C" {
    pub fn sem_init(sm: *mut semaphore_t, count: c_int) -> c_int;
    pub fn sem_destroy(sem: *mut semaphore_t) -> c_int;
    pub fn sem_wait(sem: *mut semaphore_t) -> c_int;
    pub fn sem_wait_timed(sem: *mut semaphore_t, timeout: c_int) -> c_int;
    pub fn sem_trywait(sem: *mut semaphore_t) -> c_int;
    pub fn sem_signal(sem: *mut semaphore_t) -> c_int;
    pub fn sem_count(sem: *mut semaphore_t) -> c_int;
}
