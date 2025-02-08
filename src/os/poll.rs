// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub type nfds_t = u32;

#[repr(C)]
pub struct pollfd {
    pub fd:         c_int,
    pub events:     c_short,
    pub revents:    c_short,
}

pub const POLLRDNORM: c_short   = 1 << 0;
pub const POLLRDBAND: c_short   = 1 << 1;
pub const POLLPRI: c_short      = 1 << 2;
pub const POLLOUT: c_short      = 1 << 3;
pub const POLLWRNORM: c_short   = POLLOUT;
pub const POLLWRBAND: c_short   = 1 << 4;
pub const POLLERR: c_short      = 1 << 5;
pub const POLLHUP: c_short      = 1 << 6;
pub const POLLNVAL: c_short     = 1 << 7;

pub const POLLIN: c_short       = POLLRDNORM | POLLRDBAND;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn poll(fds: *mut pollfd, nfds: nfds_t, timeout: c_int) -> c_int;
}
