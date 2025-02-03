// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub const CT_ANY: i8            = -1;
pub const CT_VGA: i8            = 0;
pub const CT_NONE: i8           = 1;
pub const CT_RGB: i8            = 2;
pub const CT_COMPOSITE: i8      = 3;

#[repr(C)]
pub enum vid_pixel_mode_t {
    PM_RGB555   = 0,
    PM_RGB565   = 1,
    PM_RGB888P  = 2,
    PM_RGB0888  = 3,
}

pub const PM_RGB888: vid_pixel_mode_t = vid_pixel_mode_t::PM_RGB0888;

#[repr(C)]
pub enum vid_display_mode_generic_t {
    DM_320x240 = 0x1000,
    DM_640x480,
    DM_256x256,
    DM_768x480,
    DM_768x576,
    DM_MULTIBUFFER
}

pub const DM_GENERIC_FIRST: vid_display_mode_generic_t = vid_display_mode_generic_t::DM_320x240;
pub const DM_GENERIC_LAST: vid_display_mode_generic_t = vid_display_mode_generic_t::DM_768x576;

pub const DM_MULTIBUFFER: u32   = 0x2000;

#[repr(C)]
pub enum vid_display_mode_t {
    DM_INVALID = 0,
    DM_320x240_VGA = 1,
    DM_320x240_NTSC,
    DM_640x480_VGA,
    DM_640x480_NTSC_IL,
    DM_640x480_PAL_IL,
    DM_256x256_PAL_IL,
    DM_768x480_NTSC_IL,
    DM_768x576_PAL_IL,
    DM_768x480_PAL_IL,
    DM_320x240_PAL,
    DM_SENTINEL,
    DM_MODE_COUNT,
}

pub const VID_INTERLACE: u32    = 0x00000001;
pub const VID_LINEDOUBLE: u32   = 0x00000002;
pub const VID_PIXELDOUBLE: u32  = 0x00000004;
pub const VID_PAL: u32          = 0x00000008;

#[repr(C)]
pub struct vid_mode_t {
    pub generic:    u16,
    pub width:      u16,
    pub height:     u16,
    pub flags:      u32,
    pub cable_type: i16,
    pub pm:         vid_pixel_mode_t,
    pub scanlines:  u16,
    pub clocks:     u16,
    pub bitmapx:    u16,
    pub bitmapy:    u16,
    pub borderx1:   u16,
    pub borderx2:   u16,
    pub bordery1:   u16,
    pub bordery2:   u16,
    pub fb_curr:    u16,
    pub fb_count:   u16,
    pub fb_size:    c_size_t,
}

#[link(name = "kallisti")]
unsafe extern "C" {
    pub static mut vram_s: *mut u16;
    pub static mut vram_l: *mut u32;
    pub fn vid_check_cable() -> i8;
    pub fn vid_set_vram(base: u32);
    pub fn vid_set_start(base: u32);
    pub fn vid_get_start(fb: u32) -> u32;
    pub fn vid_set_fb(fb: i32);
    pub fn vid_flip(fb: i32);
    pub fn vid_border_color(r: u8, g: u8, b: u8) -> u32;
    pub fn vid_clear(r: u8, g: u8, b: u8);
    pub fn vid_empty();
    pub fn vid_get_enabled() -> bool;
    pub fn vid_set_enabled(val: bool);
    pub fn vid_waitvbl();
    pub fn vid_set_mode(dm: c_int, pm: vid_pixel_mode_t);
    pub fn vid_set_mode_ex(mode: *mut vid_mode_t);
    pub fn vid_init(disp_mode: c_int, pixel_mode: vid_pixel_mode_t);
    pub fn vid_shutdown();
    pub fn vid_screen_shot(destfn: *const c_char) -> c_int;
    pub fn vid_screen_shot_data(buffer: *mut *mut u8) -> c_size_t;
}
