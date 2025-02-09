// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

use crate::os::opts::FD_SETSIZE;

pub const NFDBITS: usize    = 32;

#[repr(C)]
pub struct fd_set {
    fds_bits:   [c_ulong; FD_SETSIZE / NFDBITS],
}

// FIXME: Implement FD_SET, FD_CLR, FD_ISSET, FD_ZERO

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn select(nfds: c_int, readfds: *mut fd_set, writefds: *mut fd_set, errorfds: *mut fd_set,
                  timeout: *mut timeval) -> c_int;
}
