// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;
use super::netinet::{in_addr, in_addr_t};
use super::socket::socklen_t;

#[link(name = "kallisti")]
extern "C" {
    pub fn htonl(value: u32) -> u32;
    pub fn ntohl(value: u32) -> u32;
    pub fn htons(value: u16) -> u16;
    pub fn ntohs(value: u16) -> u16;
    pub fn inet_addr(cp: *const c_char) -> in_addr_t;
    pub fn inet_aton(cp: *const c_char, pin: *mut in_addr) -> c_int;
    pub fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    pub fn inet_ntop(af: c_int, src: *const c_void, dst: *mut c_char,
                     size: socklen_t) -> *const c_char;
    pub fn inet_ntoa(addr: in_addr) -> *mut c_char;
}
