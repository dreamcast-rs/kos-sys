// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

use crate::os::nmmgr::nmmgr_handler_t;

#[repr(C)]
pub struct export_sym_t {
    pub name:   *const c_char,
    pub ptr:    c_uintptr_t,
}

#[repr(C)]
pub struct symtab_handler_t {
    nmmgr:      nmmgr_handler_t,
    table:      *mut export_sym_t,
}

#[link(name = "kallisti")]
unsafe extern "C" {
    pub static mut kernel_symtab: [export_sym_t; 0];
    pub static mut arch_symtab: [export_sym_t; 0];
    pub fn export_init();
    pub fn export_lookup(name: *const c_char) -> *mut export_sym_t;
    pub fn export_lookup_path(name: *const c_char,
                              path: *const c_char) -> *mut export_sym_t;
    pub fn export_lookup_addr(addr: c_uintptr_t) -> *mut export_sym_t;
}
