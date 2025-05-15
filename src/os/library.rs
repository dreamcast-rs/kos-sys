// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

use crate::arch::types::tid_t;
use crate::os::elf::elf_prog_t;

#[repr(C)]
pub struct klqueue {
    tqh_first:  *mut klibrary_t,
    tqh_last:   *mut *mut klibrary_t,
}

#[repr(C)]
pub struct kllist {
    lh_first:   *mut klibrary_t,
}

pub type libid_t = tid_t;

#[repr(C)]
pub struct list {
    le_next:    *mut klibrary_t,
    le_prev:    *mut *mut klibrary_t,
}

#[repr(C)]
pub struct klibrary_t {
    pub list:               list,
    pub libid:              libid_t,
    pub flags:              u32,
    pub image:              elf_prog_t,
    pub refcnt:             c_int,
    pub lib_get_name:       Option<unsafe extern "C" fn() -> *const c_char>,
    pub lib_get_version:    Option<unsafe extern "C" fn() -> u32>,
    pub lib_open:           Option<unsafe extern "C" fn(lib: *mut klibrary_t) -> c_int>,
    pub lib_close:          Option<unsafe extern "C" fn(lib: *mut klibrary_t) -> c_int>,
}

pub const LIBRARY_DEFAULTS: u32 = 0;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub static mut library_list: kllist;
    pub fn library_by_libid(libid: libid_t) -> *mut klibrary_t;
    pub fn library_create(flags: c_int) -> *mut klibrary_t;
    pub fn library_destroy(lib: *mut klibrary_t) -> c_int;
    pub fn library_open(name: *const c_char, r#fn: *const c_char) -> *mut klibrary_t;
    pub fn library_lookup(name: *const c_char) -> *mut klibrary_t;
    pub fn library_lookup_fn(r#fn: *const c_char) -> *mut klibrary_t;
    pub fn library_close(lib: *mut klibrary_t) -> c_int;
    pub fn library_get_libid(lib: *mut klibrary_t) -> libid_t;
    pub fn library_get_refcnt(lib: *mut klibrary_t) -> c_int;
    pub fn library_get_name(lib: *mut klibrary_t) -> *const c_char;
    pub fn library_get_version(lib: *mut klibrary_t) -> u32;
    pub fn library_init();
    pub fn library_shutdown();
}
