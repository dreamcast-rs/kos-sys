// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

use super::types::IOV_MAX;

#[repr(C)]
pub struct iovec {
    pub iov_base:   *mut c_void,
    pub iov_len:    c_size_t,
}

pub const UIO_MAXIOV: c_int = IOV_MAX;
