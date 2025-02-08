// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

use crate::os::socket::{sockaddr, socklen_t};

#[repr(C)]
pub struct hostent {
    pub h_name:         *mut c_char,
    pub h_aliases:      *mut *mut c_char,
    pub h_addrtype:     c_int,
    pub h_length:       c_int,
    pub h_addr_list:    *mut *mut c_char,
}

#[repr(C)]
pub struct addrinfo {
    pub ai_flags:       c_int,
    pub ai_family:      c_int,
    pub ai_socktype:    c_int,
    pub ai_protocol:    c_int,
    pub ai_addrlen:     socklen_t,
    pub ai_addr:        *mut sockaddr,
    pub ai_canonname:   *mut c_char,
    pub ai_next:        *mut addrinfo,
}

pub const HOST_NOT_FOUND: c_int = 1;
pub const TRY_AGAIN: c_int      = 2;
pub const NO_RECOVERY: c_int    = 3;
pub const NO_DATA: c_int        = 4;

pub const EAI_AGAIN: c_int      = 1;
pub const EAI_BADFLAGS: c_int   = 2;
pub const EAI_FAIL: c_int       = 3;
pub const EAI_FAMILY: c_int     = 4;
pub const EAI_MEMORY: c_int     = 5;
pub const EAI_NONAME: c_int     = 6;
pub const EAI_SERVICE: c_int    = 7;
pub const EAI_SOCKTYPE: c_int   = 8;
pub const EAI_SYSTEM: c_int     = 9;
pub const EAI_OVERFLOW: c_int   = 10;

pub const AI_PASSIVE: c_int     = 0x00000001;
pub const AI_CANONNAME: c_int   = 0x00000002;
pub const AI_NUMERICHOST: c_int = 0x00000004;
pub const AI_NUMERICSERV: c_int = 0x00000008;
pub const AI_V4MAPPED: c_int    = 0x00000010;
pub const AI_ALL: c_int         = 0x00000020;
pub const AI_ADDRCONFIG: c_int  = 0x00000040;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub static mut h_errno: c_int;
    pub fn freeaddrinfo(ai: *mut addrinfo);
    pub fn getaddrinfo(nodename: *const c_char, servname: *const c_char,
                       hints: *const addrinfo, res: *mut *mut addrinfo) -> c_int;
    pub fn gethostbyname(name: *const c_char) -> *mut hostent;
    pub fn gethostbyname2(name: *const c_char, af: c_int) -> *mut hostent;
}
