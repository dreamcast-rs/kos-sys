// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;
use super::maple::vmu::{VMU_SCREEN_WIDTH, VMU_SCREEN_HEIGHT};

#[repr(C)]
pub struct vmufb_t {
    pub data: [u32; super::maple::vmu::VMU_SCREEN_WIDTH],
}

#[repr(C)]
pub struct vmufb_font_t {
    pub id:         c_uint,
    pub w:          c_uint,
    pub h:          c_uint,
    pub stride:     c_size_t,
    pub fontdata:   *const c_char,
}

#[inline]
pub fn vmufb_print_string(fb: *mut vmufb_t, font: *const vmufb_font_t,
                          str: *const c_char) {
    unsafe {
        vmufb_print_string_into(fb, font, 0, 0, VMU_SCREEN_WIDTH as u32,
                                VMU_SCREEN_HEIGHT as u32, 0, str);
    }
}

#[link(name = "kallisti")]
extern "C" {
    pub fn vmufb_paint_area(fb: *mut vmufb_t, x: c_uint, y: c_uint,
                            w: c_uint, h: c_uint, data: *const c_char);
    pub fn vmufb_clear_area(fb: *mut vmufb_t, x: c_uint, y: c_uint, w: c_uint, h: c_uint);
    pub fn vmufb_clear(fb: *mut vmufb_t);
    pub fn vmufb_present(fb: *const vmufb_t, dev: *mut super::maple::maple_device_t);
    pub fn vmufb_print_string_into(fb: *mut vmufb_t, font: *const vmufb_font_t, x: c_uint,
                                   y: c_uint, w: c_uint, h: c_uint, line_spacing: c_uint,
                                   str: *const c_char);
    pub fn vmu_printf(fmt: *const c_char, ...);
    pub fn vmu_set_font(font: *const vmufb_font_t) -> *const vmufb_font_t;
    pub fn vmu_get_font() -> *const vmufb_font_t;
}
