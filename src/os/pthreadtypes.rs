// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub type pthread_t = c_ulong;

#[repr(C)]
pub struct pthread_mutexattr_t {
    pub mtype:      c_int,
    pub robust:     c_int,
}

#[repr(C)]
pub struct pthread_rwlockattr_t {
    _unused:        [c_char; 0],
}

pub const __PTHREAD_HAVE_CONDATTR_TYPE: c_int   = 1;
pub const __PTHREAD_CONDATTR_SIZE: usize        = 16;

#[repr(C)]
pub union pthread_condattr_t {
    pub __data:     [c_uchar; __PTHREAD_CONDATTR_SIZE],
    pub __align:    c_long,
}

#[repr(C)]
pub struct pthread_barrierattr_t {
    _unused:        [c_char; 0],
}

pub const __PTHREAD_HAVE_ATTR_TYPE: c_int       = 1;
pub const __PTHREAD_ATTR_SIZE: usize            = 32;

#[repr(C)]
pub union pthread_attr_t {
    pub __data:     [c_uchar; __PTHREAD_ATTR_SIZE],
    pub __align:    c_long,
}

pub const __PTHREAD_HAVE_MUTEX_TYPE: c_int      = 1;
pub const __PTHREAD_MUTEX_SIZE: usize           = 32;

#[repr(C)]
pub union pthread_mutex_t {
    pub __data:     [c_uchar; __PTHREAD_MUTEX_SIZE],
    pub __align:    c_long,
}

pub const __PTHREAD_HAVE_COND_TYPE: c_int       = 1;
pub const __PTHREAD_COND_SIZE: usize            = 16;

#[repr(C)]
pub union pthread_cond_t {
    pub __data:     [c_uchar; __PTHREAD_COND_SIZE],
    pub __align:    c_long,
}

pub const __PTHREAD_HAVE_RWLOCK_TYPE: c_int     = 1;
pub const __PTHREAD_RWLOCK_SIZE: usize          = 32;

#[repr(C)]
pub union pthread_rwlock_t {
    pub __data:     [c_uchar; __PTHREAD_RWLOCK_SIZE],
    pub __align:    c_long,
}

pub const __PTHREAD_HAVE_BARRIER_TYPE: c_int    = 1;
pub const __PTHREAD_BARRIER_SIZE: usize         = 64;

#[repr(C)]
pub union pthread_barrier_t {
    pub __data:     [c_uchar; __PTHREAD_BARRIER_SIZE],
    pub __align:    c_long,
}
