// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[repr(C)]
pub enum pvr_color_fmts {
    PVR_CLRFMT_ARGBPACKED,
    PVR_CLRFMT_4FLOATS,
    PVR_CLRFMT_INTENSITY,
    PVR_CLRFMT_INTENSITY_PREV,
}

#[repr(C)]
pub enum pvr_clip_mode {
    PVR_USERCLIP_DISABLE    = 0,
    PVR_USERCLIP_INSIDE     = 2,
    PVR_USERCLIP_OUTSIDE    = 3,
}

#[repr(C)]
pub enum pvr_list_type {
    PVR_LIST_OP_POLY,
    PVR_LIST_OP_MOD,
    PVR_LIST_TR_POLY,
    PVR_LIST_TR_MOD,
    PVR_LIST_PT_POLY,
}

#[repr(C)]
pub enum pvr_cull_mode {
    PVR_CULLING_NONE,
    PVR_CULLING_SMALL,
    PVR_CULLING_CCW,
    PVR_CULLING_CW,
}

#[repr(C)]
pub enum pvr_depthcmp_mode {
    PVR_DEPTHCMP_NEVER,
    PVR_DEPTHCMP_LESS,
    PVR_DEPTHCMP_EQUAL,
    PVR_DEPTHCMP_LEQUAL,
    PVR_DEPTHCMP_GREATER,
    PVR_DEPTHCMP_NOTEQUAL,
    PVR_DEPTHCMP_GEQUAL,
    PVR_DEPTHCMP_ALWAYS,
}

#[repr(C)]
pub enum pvr_uv_size {
    PVR_UV_SIZE_8,
    PVR_UV_SIZE_16,
    PVR_UV_SIZE_32,
    PVR_UV_SIZE_64,
    PVR_UV_SIZE_128,
    PVR_UV_SIZE_256,
    PVR_UV_SIZE_512,
    PVR_UV_SIZE_1024,
}

#[repr(C)]
pub enum pvr_txr_shading_mode {
    PVR_TXRENV_REPLACE,
    PVR_TXRENV_MODULATE,
    PVR_TXRENV_DECAL,
    PVR_TXRENV_MODULATEALPHA,
}

pub type pvr_filter_mode = c_int;
pub const PVR_FILTER_NEAREST: pvr_filter_mode       = 0;
pub const PVR_FILTER_BILINEAR: pvr_filter_mode      = 1;
pub const PVR_FILTER_TRILINEAR1: pvr_filter_mode    = 2;
pub const PVR_FILTER_TRILINEAR2: pvr_filter_mode    = 3;
pub const PVR_FILTER_NONE: pvr_filter_mode          = PVR_FILTER_NEAREST;

#[repr(C)]
pub enum pvr_fog_type {
    PVR_FOG_TABLE,
    PVR_FOG_VERTEX,
    PVR_FOG_DISABLE,
    PVR_FOG_TABLE2,
}

#[repr(C)]
pub enum pvr_blend_mode {
    PVR_BLEND_ZERO,
    PVR_BLEND_ONE,
    PVR_BLEND_DESTCOLOR,
    PVR_BLEND_INVDESTCOLOR,
    PVR_BLEND_SRCALPHA,
    PVR_BLEND_INVSRCALPHA,
    PVR_BLEND_DESTALPHA,
    PVR_BLEND_INVDESTALPHA,
}

#[repr(C)]
pub enum pvr_pixel_mode {
    PVR_PIXEL_MODE_ARGB1555,
    PVR_PIXEL_MODE_RGB565,
    PVR_PIXEL_MODE_ARGB4444,
    PVR_PIXEL_MODE_YUV422,
    PVR_PIXEL_MODE_BUMP,
    PVR_PIXEL_MODE_PAL_4BPP,
    PVR_PIXEL_MODE_PAL_8BPP,
}


#[repr(C)]
pub enum pvr_strip_len {
    PVR_STRIP_LEN_1,
    PVR_STRIP_LEN_2,
    PVR_STRIP_LEN_4,
    PVR_STRIP_LEN_6,
}

#[repr(C)]
pub enum pvr_hdr_type {
    PVR_HDR_EOL             = 0,
    PVR_HDR_USERCLIP        = 1,
    PVR_HDR_OBJECT_LIST_SET = 2,
    PVR_HDR_POLY            = 4,
    PVR_HDR_SPRITE          = 5,
}

pub type pvr_txr_ptr_t = u32;

#[inline]
pub fn to_pvr_txr_ptr(addr: super::pvr_ptr_t) -> pvr_txr_ptr_t {
    ((addr as u32) & 0x00fffff8) >> 3
}

// FIXME: The pvr_poly_hdr_cmd type is incomplete

#[repr(C)]
pub struct pvr_poly_hdr_cmd {
    bits:       u32,
}

// FIXME: The pvr_poly_hdr_mode1 type is incomplete

#[repr(C)]
pub struct pvr_poly_hdr_mode1 {
    bits:       u32,
}

// FIXME: The pvr_poly_hdr_mode2 type is incomplete

#[repr(C)]
pub struct pvr_poly_hdr_mode2 {
    bits:       u32,
}

// FIXME: The pvr_poly_hdr_mode3 type is incomplete

#[repr(C)]
pub struct pvr_poly_hdr_mode3 {
    bits:       u32,
}

// FIXME: The pvr_poly_hdr_t type is incomplete

#[repr(C, align(32))]
pub struct pvr_poly_hdr_t {
    cmd:        pvr_poly_hdr_cmd,
    mode1:      pvr_poly_hdr_mode1,
    mode2:      pvr_poly_hdr_mode2,
    mode3:      pvr_poly_hdr_mode3,
    // dummy vals
    d1:          u32,
    d2:          u32,
    d3:          u32,
    d4:          u32,
}
