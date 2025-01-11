// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

// "self" parameters renamed to "dev"

#[repr(C)]
pub struct ppp_device_t {
    pub name:       *const c_char,
    pub descr:      *const c_char,
    pub index:      c_int,
    pub flags:      u32,
    pub privdata:   *mut c_void,
    pub detect:     Option<unsafe extern "C" fn(dev: *mut ppp_device_t) -> c_int>,
    pub init:       Option<unsafe extern "C" fn(dev: *mut ppp_device_t) -> c_int>,
    pub shutdown:   Option<unsafe extern "C" fn(dev: *mut ppp_device_t) -> c_int>,
    pub tx:         Option<unsafe extern "C" fn(dev: *mut ppp_device_t, data: *const u8,
                                                len: c_size_t, flags: u32) -> c_int>,
    pub rx:         Option<unsafe extern "C" fn(dev: *mut ppp_device_t,
                                                out_len: *mut c_ssize_t) -> *const u8>,
}

pub const PPP_TX_END_OF_PKT: u32        = 0x00000001;

// "self" parameters renamed to "proto"

#[repr(C)]
pub struct ppp_protocol_t {
    pub next:           *mut ppp_protocol_t,
    pub prev:           *mut *mut ppp_protocol_t,
    pub name:           *const c_char,
    pub code:           u16,
    pub privdata:       *mut c_void,
    pub init:           Option<unsafe extern "C" fn(proto: *mut ppp_protocol_t) -> c_int>,
    pub shutdown:       Option<unsafe extern "C" fn(proto: *mut ppp_protocol_t) -> c_int>,
    pub input:          Option<unsafe extern "C" fn(proto: *mut ppp_protocol_t,
                                                    buf: *const u8, len: c_size_t)
                                                    -> c_int>,
    pub enter_phase:    Option<unsafe extern "C" fn(proto: *mut ppp_protocol_t,
                                                    oldp: c_int, newp: c_int)>,
    pub check_timeouts: Option<unsafe extern "C" fn(proto: *mut ppp_protocol_t, tm: u64)>,
}

pub const PPP_PHASE_DEAD: c_int         = 0x01;
pub const PPP_PHASE_ESTABLISH: c_int    = 0x02;
pub const PPP_PHASE_AUTHENTICATE: c_int = 0x03;
pub const PPP_PHASE_NETWORK: c_int      = 0x04;
pub const PPP_PHASE_TERMINATE: c_int    = 0x05;

pub const PPP_FLAG_AUTH_PAP: u32        = 0x00000001;
pub const PPP_FLAG_AUTH_CHAP: u32       = 0x00000002;
pub const PPP_FLAG_PCOMP: u32           = 0x00000004;
pub const PPP_FLAG_ACCOMP: u32          = 0x00000008;
pub const PPP_FLAG_MAGIC_NUMBER: u32    = 0x00000010;
pub const PPP_FLAG_WANT_MRU: u32        = 0x00000020;
pub const PPP_FLAG_NO_ACCM: u32         = 0x00000040;

#[link(name = "kallisti")]
extern "C" {
    pub fn ppp_set_device(dev: *mut ppp_device_t) -> c_int;
    pub fn ppp_set_login(username: *const c_char, password: *const c_char) -> c_int;
    pub fn ppp_send(data: *const u8, len: c_size_t, proto: u16) -> c_int;
    pub fn ppp_add_protocol(hnd: *mut ppp_protocol_t) -> c_int;
    pub fn ppp_del_protocol(hnd: *mut ppp_protocol_t) -> c_int;
    pub fn ppp_lcp_send_proto_reject(proto: u16, pkt: *const u8, len: c_size_t) -> c_int;
    pub fn ppp_get_flags() -> u32;
    pub fn ppp_get_peer_flags() -> u32;
    pub fn ppp_set_flags(flags: u32);
    pub fn ppp_connect() -> c_int;
    pub fn ppp_scif_init(bps: c_int) -> c_int;
    pub fn ppp_modem_init(number: *const c_char, blind: c_int,
                          conn_rate: *mut c_int) -> c_int;
    pub fn ppp_init() -> c_int;
    pub fn ppp_shutdown() -> c_int;
}
