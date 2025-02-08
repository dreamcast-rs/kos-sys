// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub const CT_ANY: i8                                    = -1;
pub const CT_VGA: i8                                    = 0;
pub const CT_NONE: i8                                   = 1;
pub const CT_RGB: i8                                    = 2;
pub const CT_COMPOSITE: i8                              = 3;

pub type vid_pixel_mode_t = c_int;
pub const PM_RGB555: vid_pixel_mode_t                   = 0;
pub const PM_RGB565: vid_pixel_mode_t                   = 1;
pub const PM_RGB888P: vid_pixel_mode_t                  = 2;
pub const PM_RGB0888: vid_pixel_mode_t                  = 3;
pub const PM_RGB888: vid_pixel_mode_t                   = 3;

pub type vid_display_mode_generic_t = c_int;
pub const DM_GENERIC_FIRST: vid_display_mode_generic_t  = 0x1000;
pub const DM_320x240: vid_display_mode_generic_t        = 0x1000;
pub const DM_640x480: vid_display_mode_generic_t        = 0x1001;
pub const DM_256x256: vid_display_mode_generic_t        = 0x1002;
pub const DM_768x480: vid_display_mode_generic_t        = 0x1003;
pub const DM_768x576: vid_display_mode_generic_t        = 0x1004;
pub const DM_GENERIC_LAST: vid_display_mode_generic_t   = DM_768x576;

pub const DM_MULTIBUFFER: vid_display_mode_generic_t    = 0x2000;

pub type vid_display_mode_t = c_int;
pub const DM_INVALID: vid_display_mode_t                = 0;
pub const DM_320x240_VGA: vid_display_mode_t            = 1;
pub const DM_320x240_NTSC: vid_display_mode_t           = 2;
pub const DM_640x480_VGA: vid_display_mode_t            = 3;
pub const DM_640x480_NTSC_IL: vid_display_mode_t        = 4;
pub const DM_640x480_PAL_IL: vid_display_mode_t         = 5;
pub const DM_256x256_PAL_IL: vid_display_mode_t         = 6;
pub const DM_768x480_NTSC_IL: vid_display_mode_t        = 7;
pub const DM_768x576_PAL_IL: vid_display_mode_t         = 8;
pub const DM_768x480_PAL_IL: vid_display_mode_t         = 9;
pub const DM_320x240_PAL: vid_display_mode_t            = 10;
pub const DM_SENTINEL: vid_display_mode_t               = 11;
pub const DM_MODE_COUNT: usize                          = 12;

pub const VID_INTERLACE: u32                            = 0x00000001;
pub const VID_LINEDOUBLE: u32                           = 0x00000002;
pub const VID_PIXELDOUBLE: u32                          = 0x00000004;
pub const VID_PAL: u32                                  = 0x00000008;

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
    pub static mut vid_builtin: [vid_mode_t; DM_MODE_COUNT as usize];
    pub static mut vid_mode: *mut vid_mode_t;
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
