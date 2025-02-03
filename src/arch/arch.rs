// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub const PAGESIZE: u32                     = 4096;
pub const PAGESIZE_BITS: u32                = 12;
pub const PAGEMASK: u32                     = PAGESIZE - 1;

#[inline]
pub fn page_count() -> usize {
    unsafe {
        ((_arch_mem_top - page_phys_base) / PAGESIZE) as usize
    }
}

pub const page_phys_base: u32               = 0x8c010000;

pub const THD_SCHED_HZ: c_uint              = 100;

pub const THD_STACK_SIZE: c_size_t          = 32768;

pub const THD_KERNEL_STACK_SIZE: c_size_t   = 64 * 1024;

pub const DEFAULT_VID_MODE: c_int           = crate::dc::video::vid_display_mode_generic_t::DM_640x480 as c_int;

pub const DEFAULT_PIXEL_MODE: c_int         = crate::dc::video::vid_pixel_mode_t::PM_RGB565 as c_int;

pub const DEFAULT_SERIAL_BAUD: c_int        = 115200;

pub const DEFAULT_SERIAL_FIFO: c_int        = 1;

pub const ELF_SYM_PREFIX: &CStr             = c"_";

pub const ELF_SYM_PREFIX_LEN: u32           = 1;

pub const ARCH_EXIT_RETURN: c_int           = 1;
pub const ARCH_EXIT_MENU: c_int             = 2;
pub const ARCH_EXIT_REBOOT: c_int           = 3;

pub const HW_MEM_16: c_size_t               = 16777216;
pub const HW_MEM_32: c_size_t               = 33554432;

#[inline]
pub fn HW_MEMSIZE() -> usize {
    unsafe {
        (_arch_mem_top - 0x8c000000) as usize
    }
}

#[inline]
pub fn DBL_MEM() -> bool {
    unsafe {
        (_arch_mem_top - 0x8d000000) != 0
    }
}

pub const HW_TYPE_RETAIL: c_int             = 0x0;
pub const HW_TYPE_SET5: c_int               = 0x9;

pub const HW_REGION_UNKNOWN: c_int          = 0x0;
pub const HW_REGION_ASIA: c_int             = 0x1;
pub const HW_REGION_US: c_int               = 0x4;
pub const HW_REGION_EUROPE: c_int           = 0xC;

#[inline]
pub fn arch_valid_address(ptr: c_uintptr_t) -> bool {
    unsafe {
        ptr >= 0x8c010000 && ptr < _arch_mem_top as usize
    }
}

#[inline]
pub fn arch_valid_text_address(ptr: c_uintptr_t) -> bool {
    (ptr as usize) >= (&raw const _executable_start as usize)
        && (ptr as usize) < (&raw const _etext as usize)
}

#[link(name = "kallisti")]
unsafe extern "C" {
    pub static mut _arch_mem_top: u32;
    pub static mut _executable_start: c_char;
    pub static mut _etext: c_char;
    pub fn arch_panic(str: *const c_char) -> !;
    pub fn arch_main() -> !;
    pub fn arch_set_exit_path(path: c_int);
    pub fn arch_exit() -> !;
    pub fn arch_return(ret_code: c_int) -> !;
    pub fn arch_abort() -> !;
    pub fn arch_reboot() -> !;
    pub fn arch_menu() -> !;
    pub fn mm_init() -> c_int;
    pub fn mm_sbrk(increment: c_ulong) -> *mut c_void;
    pub fn arch_real_exit(ret_code: c_int) -> !;
    pub fn hardware_sys_init() -> c_int;
    pub fn hardware_periph_init() -> c_int;
    pub fn hardware_shutdown();
    pub fn hardware_sys_mode(region: *mut c_int) -> c_int;
    pub fn kos_get_banner() -> *const c_char;
    pub fn kos_get_license() -> *const c_char;
    pub fn kos_get_authors() -> *const c_char;
}
