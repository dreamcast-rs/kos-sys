// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub type off_t = c_long;
pub type dev_t = c_short;
pub type uid_t = c_ushort;
pub type gid_t = c_ushort;
pub type off64_t = c_longlong;
pub type fpos_t = c_long;
pub type fpos64_t = off64_t;
pub type wint_t = c_uint;

#[repr(C)]
pub union __value {
    pub __wch:  wint_t,
    pub __wchb: [c_uchar; 4],
}

#[repr(C)]
pub struct mbstate_t {
    __count:    c_int,
    __value:    __value,
}

pub type iconv_t = *mut c_void;
pub type blkcnt_t = c_long;
pub type blksize_t = c_long;
pub type daddr_t = c_long;
pub type fsblkcnt_t = c_ulonglong;
pub type fsfilcnt_t = c_ulong;
pub type id_t = c_ulong;
pub type ino_t = c_ulong;
pub type pid_t = c_int;
pub type key_t = c_long;
pub type mode_t = c_ulong;
pub type nlink_t = c_ushort;
pub type suseconds_t = c_long;
pub type useconds_t = c_ulong;
pub type time_t = c_longlong;
pub type clockid_t = c_ulong;
pub type timer_t = c_ulong;
pub type clock_t = c_ulong;

pub const LITTLE_ENDIAN: c_int          = 1234;
pub const BIG_ENDIAN: c_int             = 4321;
pub const PDP_ENDIAN: c_int             = 3412;
pub const AT_EACCESS: c_int             = 1;
pub const AT_SYMLINK_NOFOLLOW: c_int    = 2;
pub const AT_SYMLINK_FOLLOW: c_int      = 4;
pub const AT_REMOVEDIR: c_int           = 8;
pub const IOV_MAX: c_int                = 1024;
