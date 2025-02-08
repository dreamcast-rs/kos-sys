// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[repr(C)]
pub struct mallinfo {
    pub arena:      c_int,
    pub ordblks:    c_int,
    pub smblks:     c_int,
    pub hblks:      c_int,
    pub hblkhd:     c_int,
    pub usmblks:    c_int,
    pub fsmblks:    c_int,
    pub uordblks:   c_int,
    pub fordblks:   c_int,
    pub keepcost:   c_int,
}

pub const M_MXFAST: c_int               = 1;
pub const DEFAULT_MXFAST: c_int         = 64;

pub const M_TRIM_THRESHOLD: c_int       = -1;
pub const DEFAULT_TRIM_THRESHOLD: c_int = 256 * 1024;

pub const M_TOP_PAD: c_int              = -2;
pub const DEFAULT_TOP_PAD: c_int        = 0;

pub const M_MMAP_THRESHOLD: c_int       = -3;
pub const DEFAULT_MMAP_THRESHOLD: c_int = 256 * 1024;

pub const M_MMAP_MAX: c_int             = -4;
pub const DEFAULT_MMAP_MAX: c_int       = 65536;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn malloc(size: c_size_t) -> *mut c_void;
    pub fn calloc(nmemb: c_size_t, size: c_size_t) -> *mut c_void;
    pub fn free(ptr: *mut c_void);
    pub fn realloc(ptr: *mut c_void, size: c_size_t);
    pub fn memalign(alignment: c_size_t, size: c_size_t) -> *mut c_void;
    pub fn valloc(size: c_size_t) -> *mut c_void;
    pub fn aligned_alloc(alignment: c_size_t, size: c_size_t) -> *mut c_void;
    pub fn mallinfo() -> mallinfo;
    pub fn mallopt(_: c_int, _: c_int) -> c_int;
    pub fn malloc_stats();
    pub fn malloc_irq_safe() -> c_int;
    // FIXME: Gate the following two functions behind KM_DBG
    pub fn mem_check_block(p: *mut c_void) -> c_int;
    pub fn mem_check_all() -> c_int;
}
