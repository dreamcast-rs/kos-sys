#![allow(non_snake_case)]

use crate::prelude::*;

#[repr(C)]
pub struct kos_img_t {
    pub data:       *mut c_void,
    pub w:          u32,
    pub h:          u32,
    pub fmt:        u32,
    pub byte_count: u32,
}

#[inline]
pub const fn KOS_IMG_FMT_I(x: u32) -> u32 {
    x & 0xFFFF
}

#[inline]
pub const fn KOS_IMG_FMT_D(x: u32) -> u32 {
    (x >> 16) & 0xFFFF
}

#[inline]
pub const fn KOS_IMG_FMT(i: u32, d: u32) -> u32 {
    (i & 0xFFFF) | ((d & 0xFFFF) << 16)
}

pub const KOS_IMG_FMT_NONE: u32     = 0x00;
pub const KOS_IMG_FMT_RGB888: u32   = 0x01;
pub const KOS_IMG_FMT_ARGB8888: u32 = 0x02;
pub const KOS_IMG_FMT_RGB565: u32   = 0x03;
pub const KOS_IMG_FMT_ARGB4444: u32 = 0x04;
pub const KOS_IMG_FMT_ARGB1555: u32 = 0x05;
pub const KOS_IMG_FMT_PAL4BPP: u32  = 0x06;
pub const KOS_IMG_FMT_PAL8BPP: u32  = 0x07;
pub const KOS_IMG_FMT_YUV422: u32   = 0x08;
pub const KOS_IMG_FMT_BGR565: u32   = 0x09;
pub const KOS_IMG_FMT_RGBA8888: u32 = 0x10;
pub const KOS_IMG_FMT_MASK: u32     = 0xFF;
pub const KOS_IMG_INVERTED_X: u32   = 0x0100;
pub const KOS_IMG_INVERTED_Y: u32   = 0x0200;

pub const KOS_IMG_NOT_OWNER: u32    = 0x0400;

extern "C" {
    pub fn kos_img_free(img: *mut kos_img_t, struct_also: c_int);
}
