// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

use super::thread::kthread_t;

#[repr(C)]
pub struct rw_semaphore_t {
    pub dynamic:        c_int,
    pub read_count:     c_int,
    pub write_lock:     *mut kthread_t,
    pub reader_waiting: *mut kthread_t,
}

pub const RWSEM_INITIALIZER: rw_semaphore_t = rw_semaphore_t {
                                                  dynamic: 0,
                                                  read_count: 0,
                                                  write_lock: core::ptr::null_mut(),
                                                  reader_waiting: core::ptr::null_mut()
                                              };

#[link(name = "kallisti")]
extern "C" {
    pub fn rwsem_init(s: *mut rw_semaphore_t) -> c_int;
    pub fn rwsem_destroy(s: *mut rw_semaphore_t) -> c_int;
    pub fn rwsem_read_lock_timed(s: *mut rw_semaphore_t, timeout: c_int) -> c_int;
    pub fn rwsem_read_lock(s: *mut rw_semaphore_t) -> c_int;
    pub fn rwsem_read_lock_irqsafe(s: *mut rw_semaphore_t) -> c_int;
    pub fn rwsem_write_lock_timed(s: *mut rw_semaphore_t, timeout: c_int) -> c_int;
    pub fn rwsem_write_lock(s: *mut rw_semaphore_t) -> c_int;
    pub fn rwsem_write_lock_irqsafe(s: *mut rw_semaphore_t) -> c_int;
    pub fn rwsem_read_unlock(s: *mut rw_semaphore_t) -> c_int;
    pub fn rwsem_write_unlock(s: *mut rw_semaphore_t) -> c_int;
    pub fn rwsem_unlock(s: *mut rw_semaphore_t) -> c_int;
    pub fn rwsem_read_trylock(s: *mut rw_semaphore_t) -> c_int;
    pub fn rwsem_write_trylock(s: *mut rw_semaphore_t) -> c_int;
    pub fn rwsem_read_upgrade_timed(s: *mut rw_semaphore_t, timeout: c_int) -> c_int;
    pub fn rwsem_read_upgrade(s: *mut rw_semaphore_t) -> c_int;
    pub fn rwsem_read_tryupgrade(s: *mut rw_semaphore_t) -> c_int;
    pub fn rwsem_read_count(s: *mut rw_semaphore_t) -> c_int;
    pub fn rwsem_write_locked(s: *mut rw_semaphore_t) -> c_int;
}
