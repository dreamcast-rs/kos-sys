// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

use crate::os::fs::file_t;
use crate::os::net::netif_t;
use crate::os::socket::{sockaddr, socklen_t};

#[repr(C)]
pub struct sock_list {
    pub le_next:    *mut net_socket_t,
    pub le_prev:    *mut *mut net_socket_t,
}

#[repr(C)]
pub struct net_socket_t {
    pub sock_list:  sock_list,
    pub fd:         file_t,
    pub protocol:   *mut fs_socket_proto_t,
    pub data:       *mut c_void,
}

#[repr(C)]
pub struct entry {
    pub tqe_next:   *mut fs_socket_proto_t,
    pub tqe_prev:   *mut *mut fs_socket_proto_t,
}

#[repr(C)]
pub struct fs_socket_proto_t {
    pub entry:      entry,
    pub domain:         c_int,
    pub r#type:         c_int,
    pub protocol:       c_int,
    pub socket:         Option<unsafe extern "C" fn(s: *mut net_socket_t, domain: c_int, protocol: c_int) -> c_int>,
    pub close:          Option<unsafe extern "C" fn(hnd: *mut net_socket_t)>,
    pub accept:         Option<unsafe extern "C" fn(s: *mut net_socket_t, addr: *mut sockaddr, alen: *mut socklen_t) -> c_int>,
    pub bind:           Option<unsafe extern "C" fn(s: *mut net_socket_t, addr: *mut sockaddr, alen: socklen_t) -> c_int>,
    pub connect:        Option<unsafe extern "C" fn(s: *mut net_socket_t, addr: *mut sockaddr, alen: socklen_t) -> c_int>,
    pub listen:         Option<unsafe extern "C" fn(s: *mut net_socket_t, backlog: c_int) -> c_int>,
    pub recvfrom:       Option<unsafe extern "C" fn(s: *mut net_socket_t, buffer: *mut c_void, len: c_size_t, flags: c_int, addr: *mut sockaddr, alen: *mut socklen_t) -> c_ssize_t>,
    pub sendto:         Option<unsafe extern "C" fn(s: *mut net_socket_t, msg: *const c_void, len: c_size_t, flags: c_int, addr: *const sockaddr, alen: socklen_t) -> c_ssize_t>,
    pub shutdownsock:   Option<unsafe extern "C" fn(s: *mut net_socket_t, how: c_int) -> c_int>,
    pub input:          Option<unsafe extern "C" fn(src: *mut netif_t, domain: c_int, hdr: *const c_void, data: *const u8, size: c_size_t) -> c_int>,
    pub getsockopt:     Option<unsafe extern "C" fn(s: *mut net_socket_t, level: c_int, option_name: c_int, option_value: *mut c_void, option_len: *mut socklen_t) -> c_int>,
    pub setsockopt:     Option<unsafe extern "C" fn(s: *mut net_socket_t, level: c_int, option_name: c_int, option_value: *const c_void, option_len: socklen_t) -> c_int>,
    pub getsockname:    Option<unsafe extern "C" fn(s: *mut net_socket_t, name: *mut sockaddr, name_len: *mut socklen_t) -> c_int>,
    pub getpeername:    Option<unsafe extern "C" fn(s: *mut net_socket_t, name: *mut sockaddr, name_len: *mut socklen_t) -> c_int>,
    pub fcntl:          Option<unsafe extern "C" fn(s: *mut net_socket_t, cmd: c_int, ap: VaList) -> c_int>,
    pub poll:           Option<unsafe extern "C" fn(s: *mut net_socket_t, events: c_short) -> c_short>,
}

pub const FS_SOCKET_PROTO_ENTRY: entry = entry {
                                             tqe_next: null_mut(),
                                             tqe_prev: null_mut(),
                                         };

pub const FS_SOCKET_NONBLOCK: c_int = 0x00000001;
pub const FS_SOCKET_V6ONLY: c_int   = 0x00000002;

pub const FS_SOCKET_GEN_MAX: c_int  = 0x00008000;
pub const FS_SOCKET_FAM_MAX: c_int  = 0x00800000;

#[link(name = "kallisti")]
extern "C" {
    pub fn fs_socket_init() -> c_int;
    pub fn fs_socket_shutdown() -> c_int;
    pub fn fs_socket_open_sock(proto: *mut fs_socket_proto_t) -> *mut net_socket_t;
    pub fn fs_socket_input(src: *mut netif_t, domain: c_int, protocol: c_int,
                           hdr: *const c_void, data: *const u8, size: c_size_t) -> c_int;
    pub fn fs_socket_proto_add(proto: *mut fs_socket_proto_t) -> c_int;
    pub fn fs_socket_proto_remove(proto: *mut fs_socket_proto_t) -> c_int;
}
