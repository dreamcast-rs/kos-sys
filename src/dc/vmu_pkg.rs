// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[repr(C)]
pub struct vmu_pkg_t {
    pub desc_short:         [c_char; 20],
    pub desc_long:          [c_char; 36],
    pub app_id:             [c_char; 20],
    pub icon_cnt:           c_int,
    pub icon_anim_speed:    c_int,
    pub eyecatch_type:      c_int,
    pub data_len:           c_int,
    pub icon_pal:           [u16; 16],
    pub icon_data:          *const u8,
    pub eyecatch_data:      *const u8,
    pub data:               *const u8,
}

#[repr(C)]
pub struct vmu_hdr_t {
    pub desc_short:         [c_char; 16],
    pub desc_long:          [c_char; 32],
    pub app_id:             [c_char; 16],
    pub icon_cnt:           u16,
    pub icon_anim_speed:    u16,
    pub eyecatch_type:      u16,
    pub crc:                u16,
    pub data_len:           u32,
    pub reserved:           [u8; 20],
    pub icon_pal:           [u16; 16],
}

pub const VMUPKG_EC_NONE: c_int     = 0;
pub const VMUPKG_EC_16BIT: c_int    = 1;
pub const VMUPKG_EC_256COL: c_int   = 2;
pub const VMUPKG_EC_16COL: c_int    = 3;

#[link(name = "kallisti")]
extern "C" {
    pub fn vmu_pkg_build(src: *mut vmu_pkg_t, dst: *mut *mut u8,
                         dst_size: *mut c_int) -> c_int;
    pub fn vmu_pkg_parse(data: *mut u8, pkg: *mut vmu_pkg_t) -> c_int;
}
