// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::arch::irq::irq_context_t;
use crate::prelude::*;

#[repr(C)]
pub enum ubc_address_mask_t {
    ubc_address_mask_none,
    ubc_address_mask_10,
    ubc_address_mask_12,
    ubc_address_mask_16,
    ubc_address_mask_20,
    ubc_address_mask_all,
}

#[repr(C)]
pub enum ubc_access_t {
    ubc_access_either,
    ubc_access_instruction,
    ubc_access_operand,
}

#[repr(C)]
pub enum ubc_rw_t {
    ubc_rw_either,
    ubc_rw_read,
    ubc_rw_write,
}

#[repr(C)]
pub enum ubc_size_t {
    ubc_size_any,
    ubc_size_8bit,
    ubc_size_16bit,
    ubc_size_32bit,
    ubc_size_64bit,
}

#[repr(C)]
pub struct ubc_breakpoint_instruction_t {
    pub break_before:   bool,
}

#[repr(C)]
pub struct ubc_breakpoint_data_t {
    pub enabled:        bool,
    pub value:          u32,
    pub mask:           u32,
}

#[repr(C)]
pub struct ubc_breakpoint_operand_t {
    pub rw:             ubc_rw_t,
    pub size:           ubc_size_t,
    pub data:           ubc_breakpoint_data_t,
}

#[repr(C)]
pub struct ubc_breakpoint_asid_t {
    pub enabled:        bool,
    pub value:          u8,
}

#[repr(C)]
pub struct ubc_breakpoint_t {
    pub address:        *mut c_void,
    pub address_mask:   ubc_address_mask_t,
    pub access:         ubc_access_t,
    pub instruction:    ubc_breakpoint_instruction_t,
    pub operand:        ubc_breakpoint_operand_t,
    pub asid:           ubc_breakpoint_asid_t,
    pub next:           *mut ubc_breakpoint_t,
}

pub type ubc_break_func_t = Option<unsafe extern "C" fn(bp: *const ubc_breakpoint_t,
                                                        ctx: *const irq_context_t,
                                                        user_data: *mut c_void) -> bool>;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn ubc_add_breakpoint(bp: *const ubc_breakpoint_t, callback: ubc_break_func_t,
                              user_data: *mut c_void) -> bool;
    pub fn ubc_remove_breakpoint(bp: *const ubc_breakpoint_t) -> bool;
    pub fn ubc_clear_breakpoints();
    pub fn ubc_init();
    pub fn ubc_shutdown();
}
