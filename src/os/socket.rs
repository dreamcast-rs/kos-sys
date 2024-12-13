#![allow(non_camel_case_types)]

use crate::prelude::*;

pub type socklen_t = u32;
pub type sa_family_t = u8;

#[repr(C)]
pub struct sockaddr {
    sa_family:  sa_family_t,
    sa_data:    FAM<c_char>,
}

pub const _SS_MAXSIZE: usize    = 128;

pub const _SS_ALIGNSIZE: usize  = size_of::<u64>();

pub const _SS_PAD1SIZE: usize   = _SS_ALIGNSIZE - size_of::<sa_family_t>();

pub const _SS_PAD2SIZE: usize   = _SS_MAXSIZE - size_of::<sa_family_t>()
                                  + _SS_PAD1SIZE + _SS_ALIGNSIZE;

#[repr(C)]
pub struct sockaddr_storage {
    ss_family:  sa_family_t,
    _ss_pad1:   [c_char; _SS_PAD1SIZE],
    _ss_align:  u64,
    _ss_pad2:   [c_char; _SS_PAD2SIZE],
}

pub const SOCK_DGRAM: c_int     = 1;
pub const SOCK_STREAM: c_int    = 2;

pub const SOL_SOCKET: c_int     = 1;

pub const SO_ACCEPTCONN: c_int  = 1;
pub const SO_BROADCAST: c_int   = 2;
pub const SO_DEBUG: c_int       = 3;
pub const SO_DONTROUTE: c_int   = 4;
pub const SO_ERROR: c_int       = 5;
pub const SO_KEEPALIVE: c_int   = 6;
pub const SO_LINGER: c_int      = 7;
pub const SO_OOBINLINE: c_int   = 8;
pub const SO_RCVBUF: c_int      = 9;
pub const SO_RCVLOWAT: c_int    = 10;
pub const SO_RCVTIMEO: c_int    = 11;
pub const SO_REUSEADDR: c_int   = 12;
pub const SO_SNDBUF: c_int      = 13;
pub const SO_SNDLOWAT: c_int    = 14;
pub const SO_SNDTIMEO: c_int    = 15;
pub const SO_TYPE: c_int        = 16;

pub const MSG_CTRUNC: c_int     = 0x01;
pub const MSG_DONTROUTE: c_int  = 0x02;
pub const MSG_EOR: c_int        = 0x04;
pub const MSG_OOB: c_int        = 0x08;
pub const MSG_PEEK: c_int       = 0x10;
pub const MSG_TRUNC: c_int      = 0x20;
pub const MSG_WAITALL: c_int    = 0x40;
pub const MSG_DONTWAIT: c_int   = 0x80;

pub const AF_UNSPEC: c_int      = 0;
pub const AF_INET: c_int        = 1;
pub const AF_INET6: c_int       = 2;

pub const PF_UNSPEC: c_int      = AF_UNSPEC;
pub const PF_INET: c_int        = AF_INET;
pub const PF_INET6: c_int       = AF_INET6;

pub const SHUT_RD: u32          = 0x00000001;

pub const SHUT_WR: u32          = 0x00000002;

pub const SHUT_RDWR: u32        = SHUT_RD | SHUT_WR;

pub const SOMAXCONN: c_int      = 32;

extern "C" {
    pub fn accept(socket: c_int, address: *mut sockaddr,
                  address_len: *mut socklen_t) -> c_int;
    pub fn bind(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int;
    pub fn connect(socket: c_int, address: *const sockaddr,
                   address_len: socklen_t) -> c_int;
    pub fn listen(socket: c_int, backlog: c_int) -> c_int;
    pub fn recv(socket: c_int, buffer: *mut c_void, length: c_size_t,
                flags: c_int) -> c_ssize_t;
    pub fn recvfrom(socket: c_int, buffer: *mut c_void, length: c_size_t, flags: c_int,
                    address: *mut sockaddr, address_len: *mut socklen_t) -> c_ssize_t;
    pub fn send(socket: c_int, message: *const c_void, length: c_size_t,
                flags: c_int) -> c_ssize_t;
    pub fn sendto(socket: c_int, message: *const c_void, length: c_size_t, flags: c_int,
                  dest_addr: *const sockaddr, dest_len: socklen_t) -> c_ssize_t;
    pub fn shutdown(socket: c_int, how: c_int) -> c_int;
    pub fn socket(domain: c_int, r#type: c_int, protocol: c_int) -> c_int;
    pub fn getsockname(socket: c_int, name: *mut sockaddr,
                       name_len: *mut socklen_t) -> c_int;
    pub fn getsockopt(socket: c_int, level: c_int, option_name: c_int,
                      option_value: *mut c_void, option_len: *mut socklen_t) -> c_int;
    pub fn setsockopt(socket: c_int, level: c_int, option_name: c_int,
                      option_value: *const c_void, option_len: socklen_t) -> c_int; 
}
