// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[repr(C)]
pub struct nmmgr_list_t {
    pub lh_first:   *mut nmmgr_handler_t
}

pub const NMMGR_LIST_INIT: nmmgr_list_t = nmmgr_list_t {
                                              lh_first: core::ptr::null_mut()
                                          };

#[repr(C)]
pub struct nmmgr_handler_t {
    pub pathname:   [c_char; crate::os::limits::NAME_MAX],
    pub pid:        c_int,
    pub version:    u32,
    pub flags:      u32,
    pub r#type:     u32,
    pub le_next:    *mut nmmgr_handler_t,
    pub le_prev:    *mut *mut nmmgr_handler_t,
}

#[repr(C)]
pub struct alias_handler_t {
    pub nmmgr:      nmmgr_handler_t,
    pub alias:      *mut nmmgr_handler_t,
}

pub const NMMGR_FLAGS_NEEDSFREE: u32    = 0x00000001;
pub const NMMGR_FLAGS_INDEV: u32        = 0x00000002;
pub const NMMGR_FLAGS_ALIAS: u32        = 0x00000004;

pub const NMMGR_TYPE_UNKNOWN: u32       = 0x0000;
pub const NMMGR_TYPE_VFS: u32           = 0x0010;
pub const NMMGR_TYPE_BLOCKDEV: u32      = 0x0020;
pub const NMMGR_TYPE_SINGLETON: u32     = 0x0030;
pub const NMMGR_TYPE_SYMTAB: u32        = 0x0040;
pub const NMMGR_SYS_MAX: u32            = 0x10000;

#[link(name = "kallisti")]
extern "C" {
    pub fn nmmgr_lookup(name: *const c_char) -> *mut nmmgr_handler_t;
    pub fn nmmgr_get_list() -> *mut nmmgr_list_t;
    pub fn nmmgr_handler_add(hnd: *mut nmmgr_handler_t) -> c_int;
    pub fn nmmgr_handler_remove(hnd: *mut nmmgr_handler_t) -> c_int;
    pub fn nmmgr_init();
    pub fn nmmgr_shutdown();
}
