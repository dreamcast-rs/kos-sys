// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

use crate::{BIT, GENMASK};

use mem::pvr_ptr_t;

pub mod dma;
pub mod fog;
pub mod mem;
pub mod pal;
pub mod regs;
pub mod txr;

pub type pvr_list_t = u32;

// gen struct for pvr_poly_cxt_t
#[repr(C)]
pub struct gen_poly_cxt_t {
    pub alpha:                  c_int,
    pub shading:                c_int,
    pub fog_type:               c_int,
    pub culling:                c_int,
    pub color_clamp:            c_int,
    pub clip_mode:              c_int,
    pub modifier_mode:          c_int,
    pub specular:               c_int,
    pub alpha2:                 c_int,
    pub fog_type2:              c_int,
    pub color_clamp2:           c_int,
}

// gen struct for pvr_sprite_cxt_t
#[repr(C)]
pub struct gen_sprite_cxt_t {
    pub alpha:                  c_int,
    pub fog_type:               c_int,
    pub culling:                c_int,
    pub color_clamp:            c_int,
    pub clip_mode:              c_int,
    pub specular:               c_int,
}

// blend struct for pvr_poly_cxt_t
#[repr(C)]
pub struct blend_poly_cxt_t {
    pub src:                    c_int,
    pub dst:                    c_int,
    pub src_enable:             c_int,
    pub dst_enable:             c_int,
    pub src2:                   c_int,
    pub dst2:                   c_int,
    pub src_enable2:            c_int,
    pub dst_enable2:            c_int,
}

// blend struct for pvr_sprite_cxt_t
#[repr(C)]
pub struct blend_sprite_cxt_t {
    pub src:                    c_int,
    pub dst:                    c_int,
    pub src_enable:             c_int,
    pub dst_enable:             c_int,
}

// fmt struct for pvr_poly_cxt_t
#[repr(C)]
pub struct fmt_cxt_t {
    pub color:                  c_int,
    pub uv:                     c_int,
    pub modifier:               c_int,
}

// depth struct for pvr_poly_cxt_t and pvr_sprite_cxt_t
#[repr(C)]
pub struct depth_cxt_t {
    pub comparison:             c_int,
    pub write:                  c_int,
}

// txr struct for pvr_poly_cxt_t and pvr_sprite_cxt_t
#[repr(C)]
pub struct txr_cxt_t {
    pub enable:                 c_int,
    pub filter:                 c_int,
    pub mipmap:                 c_int,
    pub mipmap_bias:            c_int,
    pub uv_flip:                c_int,
    pub uv_clamp:               c_int,
    pub alpha:                  c_int,
    pub env:                    c_int,
    pub width:                  c_int,
    pub height:                 c_int,
    pub base:                   pvr_ptr_t,
}

#[repr(C)]
pub struct pvr_poly_cxt_t {
    pub list_type:              c_int,
    pub r#gen:                  gen_poly_cxt_t,
    pub blend:                  blend_poly_cxt_t,
    pub fmt:                    fmt_cxt_t,
    pub depth:                  depth_cxt_t,
    pub txr:                    txr_cxt_t,
    pub txr2:                   txr_cxt_t,
}

#[repr(C)]
pub struct pvr_sprite_cxt_t {
    pub list_type:              c_int,
    pub r#gen:                  gen_sprite_cxt_t,
    pub blend:                  blend_sprite_cxt_t,
    pub depth:                  depth_cxt_t,
    pub txr:                    txr_cxt_t,
}


pub const PVR_LIST_OP_POLY: pvr_list_t          = 0;
pub const PVR_LIST_OP_MOD: pvr_list_t           = 1;
pub const PVR_LIST_TR_POLY: pvr_list_t          = 2;
pub const PVR_LIST_TR_MOD: pvr_list_t           = 3;
pub const PVR_LIST_PT_POLY: pvr_list_t          = 4;

pub const PVR_SHADE_FLAT: c_int                 = 0;
pub const PVR_SHADE_GOURAUD: c_int              = 1;

pub const PVR_DEPTHCMP_NEVER: c_int             = 0;
pub const PVR_DEPTHCMP_LESS: c_int              = 1;
pub const PVR_DEPTHCMP_EQUAL: c_int             = 2;
pub const PVR_DEPTHCMP_LEQUAL: c_int            = 3;
pub const PVR_DEPTHCMP_GREATER: c_int           = 4;
pub const PVR_DEPTHCMP_NOTEQUAL: c_int          = 5;
pub const PVR_DEPTHCMP_GEQUAL: c_int            = 6;
pub const PVR_DEPTHCMP_ALWAYS: c_int            = 7;

pub const PVR_CULLING_NONE: c_int               = 0;
pub const PVR_CULLING_SMALL: c_int              = 1;
pub const PVR_CULLING_CCW: c_int                = 2;
pub const PVR_CULLING_CW: c_int                 = 3;

pub const PVR_DEPTHWRITE_ENABLE: c_int          = 0;
pub const PVR_DEPTHWRITE_DISABLE: c_int         = 1;

pub const PVR_TEXTURE_DISABLE: c_int            = 0;
pub const PVR_TEXTURE_ENABLE: c_int             = 1;

pub const PVR_BLEND_ZERO: c_int                 = 0;
pub const PVR_BLEND_ONE: c_int                  = 1;
pub const PVR_BLEND_DESTCOLOR: c_int            = 2;
pub const PVR_BLEND_INVDESTCOLOR: c_int         = 3;
pub const PVR_BLEND_SRCALPHA: c_int             = 4;
pub const PVR_BLEND_INVSRCALPHA: c_int          = 5;
pub const PVR_BLEND_DESTALPHA: c_int            = 6;
pub const PVR_BLEND_INVDESTALPHA: c_int         = 7;

pub const PVR_BLEND_DISABLE: c_int              = 0;
pub const PVR_BLEND_ENABLE: c_int               = 1;

pub const PVR_FOG_TABLE: c_int                  = 0;
pub const PVR_FOG_VERTEX: c_int                 = 1;
pub const PVR_FOG_DISABLE: c_int                = 2;
pub const PVR_FOG_TABLE2: c_int                 = 3;

pub const PVR_USERCLIP_DISABLE: c_int           = 0;
pub const PVR_USERCLIP_INSIDE: c_int            = 2;
pub const PVR_USERCLIP_OUTSIDE: c_int           = 3;

pub const PVR_CLRCLAMP_DISABLE: c_int           = 0;
pub const PVR_CLRCLAMP_ENABLE: c_int            = 1;

pub const PVR_SPECULAR_DISABLE: c_int           = 0;
pub const PVR_SPECULAR_ENABLE: c_int            = 1;

pub const PVR_ALPHA_DISABLE: c_int              = 0;
pub const PVR_ALPHA_ENABLE: c_int               = 1;

pub const PVR_TXRALPHA_ENABLE: c_int            = 0;
pub const PVR_TXRALPHA_DISABLE: c_int           = 1;

pub const PVR_UVFLIP_NONE: c_int                = 0;
pub const PVR_UVFLIP_V: c_int                   = 1;
pub const PVR_UVFLIP_U: c_int                   = 2;
pub const PVR_UVFLIP_UV: c_int                  = 3;

pub const PVR_UVCLAMP_NONE: c_int               = 0;
pub const PVR_UVCLAMP_V: c_int                  = 1;
pub const PVR_UVCLAMP_U: c_int                  = 2;
pub const PVR_UVCLAMP_UV: c_int                 = 3;

pub const PVR_FILTER_NONE: c_int                = 0;
pub const PVR_FILTER_NEAREST: c_int             = 0;
pub const PVR_FILTER_BILINEAR: c_int            = 2;
pub const PVR_FILTER_TRILINEAR1: c_int          = 4;
pub const PVR_FILTER_TRILINEAR2: c_int          = 6;

pub const PVR_MIPBIAS_NORMAL: c_int             = PVR_MIPBIAS_1_00;
pub const PVR_MIPBIAS_0_25: c_int               = 1;
pub const PVR_MIPBIAS_0_50: c_int               = 2;
pub const PVR_MIPBIAS_0_75: c_int               = 3;
pub const PVR_MIPBIAS_1_00: c_int               = 4;
pub const PVR_MIPBIAS_1_25: c_int               = 5;
pub const PVR_MIPBIAS_1_50: c_int               = 6;
pub const PVR_MIPBIAS_1_75: c_int               = 7;
pub const PVR_MIPBIAS_2_00: c_int               = 8;
pub const PVR_MIPBIAS_2_25: c_int               = 9;
pub const PVR_MIPBIAS_2_50: c_int               = 10;
pub const PVR_MIPBIAS_2_75: c_int               = 11;
pub const PVR_MIPBIAS_3_00: c_int               = 12;
pub const PVR_MIPBIAS_3_25: c_int               = 13;
pub const PVR_MIPBIAS_3_50: c_int               = 14;
pub const PVR_MIPBIAS_3_75: c_int               = 15;

pub const PVR_TXRENV_REPLACE: c_int             = 0;
pub const PVR_TXRENV_MODULATE: c_int            = 1;
pub const PVR_TXRENV_DECAL: c_int               = 2;
pub const PVR_TXRENV_MODULATEALPHA: c_int       = 3;

pub const PVR_MIPMAP_DISABLE: c_int             = 0;
pub const PVR_MIPMAP_ENABLE: c_int              = 1;

pub const PVR_TXRFMT_NONE: c_int                = 0;
pub const PVR_TXRFMT_VQ_DISABLE: c_int          = 0 << 30;
pub const PVR_TXRFMT_VQ_ENABLE : c_int          = 1 << 30;
pub const PVR_TXRFMT_ARGB1555: c_int            = 0 << 27;
pub const PVR_TXRFMT_RGB565: c_int              = 1 << 27;
pub const PVR_TXRFMT_ARGB4444: c_int            = 2 << 27;
pub const PVR_TXRFMT_YUV422: c_int              = 3 << 27;
pub const PVR_TXRFMT_BUMP: c_int                = 4 << 27;
pub const PVR_TXRFMT_PAL4BPP: c_int             = 5 << 27;
pub const PVR_TXRFMT_PAL8BPP: c_int             = 6 << 27;
pub const PVR_TXRFMT_TWIDDLED: c_int            = 0 << 26;
pub const PVR_TXRFMT_NONTWIDDLED: c_int         = 1 << 26;
pub const PVR_TXRFMT_NOSTRIDE: c_int            = 0 << 21;
pub const PVR_TXRFMT_STRIDE: c_int              = 1 << 21;

#[inline]
pub const fn PVR_TXRFMT_8BPP_PAL(n: c_int) -> c_int {
    n << 25
}

#[inline]
pub const fn PVR_TXRFMT_4BPP_PAL(n: c_int) -> c_int {
    n << 21
}

pub const PVR_CLRFMT_ARGBPACKED: c_int          = 0;
pub const PVR_CLRFMT_4FLOATS: c_int             = 1;
pub const PVR_CLRFMT_INTENSITY: c_int           = 2;
pub const PVR_CLRFMT_INTENSITY_PRE: c_int       = 3;

pub const PVR_UVFMT_32BIT: c_int                = 0;
pub const PVR_UVFMT_16BIT: c_int                = 1;

pub const PVR_MODIFIER_DISABLE: c_int           = 0;
pub const PVR_MODIFIER_ENABLE: c_int            = 1;

pub const PVR_MODIFIER_CHEAP_SHADOW: c_int      = 0;
pub const PVR_MODIFIER_NORMAL: c_int            = 1;

pub const PVR_MODIFIER_OTHER_POLY: c_int        = 0;
pub const PVR_MODIFIER_INCLUDE_LAST_POLY: c_int = 1;
pub const PVR_MODIFIER_EXCLUDE_LAST_POLY: c_int = 2;

#[repr(C, align(32))]
pub struct pvr_poly_hdr_t {
    pub cmd:                    u32,
    pub mode1:                  u32,
    pub mode2:                  u32,
    pub mode3:                  u32,
    pub d1:                     u32,
    pub d2:                     u32,
    pub d3:                     u32,
    pub d4:                     u32,
}

#[repr(C, align(32))]
pub struct pvr_poly_ic_hdr_t {
    pub cmd:                    u32,
    pub mode1:                  u32,
    pub mode2:                  u32,
    pub mode3:                  u32,
    pub a:                      c_float,
    pub r:                      c_float,
    pub g:                      c_float,
    pub b:                      c_float,
}

#[repr(C, align(32))]
pub struct pvr_poly_mod_hdr_t {
    pub cmd:                    u32,
    pub mode1:                  u32,
    pub mode2_0:                u32,
    pub mode3_0:                u32,
    pub mode2_1:                u32,
    pub mode3_1:                u32,
    pub d1:                     u32,
    pub d2:                     u32,
}

#[repr(C, align(32))]
pub struct pvr_sprite_hdr_t {
    pub cmd:                    u32,
    pub mode1:                  u32,
    pub mode2:                  u32,
    pub mode3:                  u32,
    pub argb:                   u32,
    pub oargb:                  u32,
    pub d1:                     u32,
    pub d2:                     u32,
}

#[repr(C, align(32))]
pub struct pvr_mod_hdr_t {
    pub cmd:                    u32,
    pub mode1:                  u32,
    pub d1:                     u32,
    pub d2:                     u32,
    pub d3:                     u32,
    pub d4:                     u32,
    pub d5:                     u32,
    pub d6:                     u32,
}

#[repr(C, align(32))]
pub struct pvr_vertex_t {
    pub flags:                  u32,
    pub x:                      c_float,
    pub y:                      c_float,
    pub z:                      c_float,
    pub u:                      c_float,
    pub v:                      c_float,
    pub argb:                   u32,
    pub oargb:                  u32,
}

#[repr(C, align(32))]
pub struct pvr_vertex_pcm_t {
    pub flags:                  u32,
    pub x:                      c_float,
    pub y:                      c_float,
    pub z:                      c_float,
    pub argb0:                  u32,
    pub argb1:                  u32,
    pub d1:                     u32,
    pub d2:                     u32,
}

#[repr(C, align(32))]
pub struct pvr_vertex_tpcm_t {
    pub flags:                  u32,
    pub x:                      c_float,
    pub y:                      c_float,
    pub z:                      c_float,
    pub u0:                     c_float,
    pub v0:                     c_float,
    pub argb0:                  u32,
    pub oargb0:                 u32,
    pub u1:                     c_float,
    pub v1:                     c_float,
    pub argb1:                  u32,
    pub oargb1:                 u32,
    pub d1:                     u32,
    pub d2:                     u32,
    pub d3:                     u32,
    pub d4:                     u32,
}

#[repr(C, align(32))]
pub struct pvr_sprite_txr_t {
    pub flags:                  u32,
    pub ax:                     c_float,
    pub ay:                     c_float,
    pub az:                     c_float,
    pub bx:                     c_float,
    pub by:                     c_float,
    pub bz:                     c_float,
    pub cx:                     c_float,
    pub cy:                     c_float,
    pub cz:                     c_float,
    pub dx:                     c_float,
    pub dy:                     c_float,
    pub dummy:                  u32,
    pub auv:                    u32,
    pub buv:                    u32,
    pub cuv:                    u32,
}

#[repr(C, align(32))]
pub struct pvr_sprite_col_t {
    pub flags:                  u32,
    pub ax:                     c_float,
    pub ay:                     c_float,
    pub az:                     c_float,
    pub bx:                     c_float,
    pub by:                     c_float,
    pub bz:                     c_float,
    pub cx:                     c_float,
    pub cy:                     c_float,
    pub cz:                     c_float,
    pub dx:                     c_float,
    pub dy:                     c_float,
    pub d1:                     u32,
    pub d2:                     u32,
    pub d3:                     u32,
    pub d4:                     u32,
}

#[repr(C, align(32))]
pub struct pvr_modifier_vol_t {
    pub flags:                  u32,
    pub ax:                     c_float,
    pub ay:                     c_float,
    pub az:                     c_float,
    pub bx:                     c_float,
    pub by:                     c_float,
    pub bz:                     c_float,
    pub cx:                     c_float,
    pub cy:                     c_float,
    pub cz:                     c_float,
    pub d1:                     u32,
    pub d2:                     u32,
    pub d3:                     u32,
    pub d4:                     u32,
    pub d5:                     u32,
    pub d6:                     u32,
}

#[inline]
pub const fn PVR_PACK_COLOR(a: f32, r: f32, g: f32, b: f32) -> u32 {
    let a = ((a * 255.0) as u8) as u32;
    let r = ((r * 255.0) as u8) as u32;
    let g = ((g * 255.0) as u8) as u32;
    let b = ((b * 255.0) as u8) as u32;

    (a << 24) | (r << 16) | (g << 8) | b
}

#[inline]
pub const fn PVR_PACK_16BIT_UV(u: f32, v: f32) -> u32 {
    (u.to_bits() & 0xFFFF0000) | (v.to_bits() >> 16)
}

pub const PVR_CMD_POLYHDR: u32                  = 0x80840000;
pub const PVR_CMD_VERTEX: u32                   = 0xE0000000;
pub const PVR_CMD_VERTEX_EOL: u32               = 0xF0000000;
pub const PVR_CMD_USERCLIP: u32                 = 0x20000000;
pub const PVR_CMD_MODIFIER: u32                 = 0x80000000;
pub const PVR_CMD_SPRITE: u32                   = 0xA0000000;

pub const PVR_TA_CMD_TYPE_SHIFT: u32            = 24;
pub const PVR_TA_CMD_TYPE_MASK: u32             = 7 << PVR_TA_CMD_TYPE_SHIFT;

pub const PVR_TA_CMD_USERCLIP_SHIFT: u32        = 16;
pub const PVR_TA_CMD_USERCLIP_MASK: u32         = 3 << PVR_TA_CMD_USERCLIP_SHIFT;

pub const PVR_TA_CMD_CLRFMT_SHIFT: u32          = 4;
pub const PVR_TA_CMD_CLRFMT_MASK: u32           = 7 << PVR_TA_CMD_CLRFMT_SHIFT;

pub const PVR_TA_CMD_SPECULAR_SHIFT: u32        = 2;
pub const PVR_TA_CMD_SPECULAR_MASK: u32         = 1 << PVR_TA_CMD_SPECULAR_SHIFT;

pub const PVR_TA_CMD_SHADE_SHIFT: u32           = 1;
pub const PVR_TA_CMD_SHADE_MASK: u32            = 1 << PVR_TA_CMD_SHADE_SHIFT;

pub const PVR_TA_CMD_UVFMT_SHIFT: u32           = 0;
pub const PVR_TA_CMD_UVFMT_MASK: u32            = 1 << PVR_TA_CMD_UVFMT_SHIFT;

pub const PVR_TA_CMD_MODIFIER_SHIFT: u32        = 7;
pub const PVR_TA_CMD_MODIFIER_MASK: u32         = 1 << PVR_TA_CMD_MODIFIER_SHIFT;

pub const PVR_TA_CMD_MODIFIERMODE_SHIFT: u32    = 6;
pub const PVR_TA_CMD_MODIFIERMODE_MASK: u32     = 1 <<  PVR_TA_CMD_MODIFIERMODE_SHIFT;

pub const PVR_TA_PM1_DEPTHCMP_SHIFT: u32        = 29;
pub const PVR_TA_PM1_DEPTHCMP_MASK: u32         = 7 << PVR_TA_PM1_DEPTHCMP_SHIFT;

pub const PVR_TA_PM1_CULLING_SHIFT: u32         = 27;
pub const PVR_TA_PM1_CULLING_MASK: u32          = 3 << PVR_TA_PM1_CULLING_SHIFT;

pub const PVR_TA_PM1_DEPTHWRITE_SHIFT: u32      = 26;
pub const PVR_TA_PM1_DEPTHWRITE_MASK: u32       = 1 << PVR_TA_PM1_DEPTHWRITE_SHIFT;

pub const PVR_TA_PM1_TXRENABLE_SHIFT: u32       = 25;
pub const PVR_TA_PM1_TXRENABLE_MASK: u32        = 1 << PVR_TA_PM1_TXRENABLE_SHIFT;

pub const PVR_TA_PM1_MODIFIERINST_SHIFT: u32    = 29;
pub const PVR_TA_PM1_MODIFIERINST_MASK: u32     = 3 <<  PVR_TA_PM1_MODIFIERINST_SHIFT;

pub const PVR_TA_PM2_SRCBLEND_SHIFT: u32        = 29;
pub const PVR_TA_PM2_SRCBLEND_MASK: u32         = 7 << PVR_TA_PM2_SRCBLEND_SHIFT;

pub const PVR_TA_PM2_DSTBLEND_SHIFT: u32        = 26;
pub const PVR_TA_PM2_DSTBLEND_MASK: u32         = 7 << PVR_TA_PM2_DSTBLEND_SHIFT;

pub const PVR_TA_PM2_SRCENABLE_SHIFT: u32       = 25;
pub const PVR_TA_PM2_SRCENABLE_MASK: u32        = 1 << PVR_TA_PM2_SRCENABLE_SHIFT;

pub const PVR_TA_PM2_DSTENABLE_SHIFT: u32       = 24;
pub const PVR_TA_PM2_DSTENABLE_MASK: u32        = 1 << PVR_TA_PM2_DSTENABLE_SHIFT;

pub const PVR_TA_PM2_FOG_SHIFT: u32             = 22;
pub const PVR_TA_PM2_FOG_MASK: u32              = 3 << PVR_TA_PM2_FOG_SHIFT;

pub const PVR_TA_PM2_CLAMP_SHIFT: u32           = 21;
pub const PVR_TA_PM2_CLAMP_MASK: u32            = 1 << PVR_TA_PM2_CLAMP_SHIFT;

pub const PVR_TA_PM2_ALPHA_SHIFT: u32           = 20;
pub const PVR_TA_PM2_ALPHA_MASK: u32            = 1 << PVR_TA_PM2_ALPHA_SHIFT;

pub const PVR_TA_PM2_TXRALPHA_SHIFT: u32        = 19;
pub const PVR_TA_PM2_TXRALPHA_MASK: u32         = 1 << PVR_TA_PM2_TXRALPHA_SHIFT;

pub const PVR_TA_PM2_UVFLIP_SHIFT: u32          = 17;
pub const PVR_TA_PM2_UVFLIP_MASK: u32           = 3 << PVR_TA_PM2_UVFLIP_SHIFT;

pub const PVR_TA_PM2_UVCLAMP_SHIFT: u32         = 15;
pub const PVR_TA_PM2_UVCLAMP_MASK: u32          = 3 << PVR_TA_PM2_UVCLAMP_SHIFT;

pub const PVR_TA_PM2_FILTER_SHIFT: u32          = 12;
pub const PVR_TA_PM2_FILTER_MASK: u32           = 7 << PVR_TA_PM2_FILTER_SHIFT;

pub const PVR_TA_PM2_MIPBIAS_SHIFT: u32         = 8;
pub const PVR_TA_PM2_MIPBIAS_MASK: u32          = 15 << PVR_TA_PM2_MIPBIAS_SHIFT;

pub const PVR_TA_PM2_TXRENV_SHIFT: u32          = 6;
pub const PVR_TA_PM2_TXRENV_MASK: u32           = 3 << PVR_TA_PM2_TXRENV_SHIFT;

pub const PVR_TA_PM2_USIZE_SHIFT: u32            = 3;
pub const PVR_TA_PM2_USIZE_MASK: u32            = 7 << PVR_TA_PM2_USIZE_SHIFT;

pub const PVR_TA_PM2_VSIZE_SHIFT: u32           = 0;
pub const PVR_TA_PM2_VSIZE_MASK: u32            = 7 << PVR_TA_PM2_VSIZE_SHIFT;

pub const PVR_TA_PM3_MIPMAP_SHIFT: u32          = 31;
pub const PVR_TA_PM3_MIPMAP_MASK: u32           = 1 << PVR_TA_PM3_MIPMAP_SHIFT;

pub const PVR_TA_PM3_TXRFMT_SHIFT: u32          = 0;
pub const PVR_TA_PM3_TXRFMT_MASK: u32           = 0xFFFFFFFF;

pub const PVR_TA_CMD_TYPE: u32                  = GENMASK!(26, 24);
pub const PVR_TA_CMD_USERCLIP: u32              = GENMASK!(17, 16);
pub const PVR_TA_CMD_MODIFIER: u32              = BIT!(7);
pub const PVR_TA_CMD_MODIFIERMODE: u32          = BIT!(6);
pub const PVR_TA_CMD_CLRFMT: u32                = GENMASK!(5, 4);
pub const PVR_TA_CMD_TXRENABLE: u32             = BIT!(3);
pub const PVR_TA_CMD_SPECULAR: u32              = BIT!(2);
pub const PVR_TA_CMD_SHADE: u32                 = BIT!(1);
pub const PVR_TA_CMD_UVFMT: u32                 = BIT!(0);
pub const PVR_TA_PM1_DEPTHCMP: u32              = GENMASK!(31, 29);
pub const PVR_TA_PM1_CULLING: u32               = GENMASK!(28, 27);
pub const PVR_TA_PM1_DEPTHWRITE: u32            = BIT!(26);
pub const PVR_TA_PM1_TXRENABLE: u32             = BIT!(25);
pub const PVR_TA_PM1_MODIFIERINST: u32          = GENMASK!(30, 29);
pub const PVR_TA_PM2_SRCBLEND: u32              = GENMASK!(31, 29);
pub const PVR_TA_PM2_DSTBLEND: u32              = GENMASK!(28, 26);
pub const PVR_TA_PM2_SRCENABLE: u32             = BIT!(25);
pub const PVR_TA_PM2_DSTENABLE: u32             = BIT!(24);
pub const PVR_TA_PM2_FOG: u32                   = GENMASK!(23, 22);
pub const PVR_TA_PM2_CLAMP: u32                 = BIT!(21);
pub const PVR_TA_PM2_ALPHA: u32                 = BIT!(20);
pub const PVR_TA_PM2_TXRALPHA: u32              = BIT!(19);
pub const PVR_TA_PM2_UVFLIP: u32                = GENMASK!(18, 17);
pub const PVR_TA_PM2_UVCLAMP: u32               = GENMASK!(16, 15);
pub const PVR_TA_PM2_FILTER: u32                = GENMASK!(14, 12);
pub const PVR_TA_PM2_MIPBIAS: u32               = GENMASK!(11, 8);
pub const PVR_TA_PM2_TXRENV: u32                = GENMASK!(7, 6);
pub const PVR_TA_PM2_USIZE: u32                 = GENMASK!(5, 3);
pub const PVR_TA_PM2_VSIZE: u32                 = GENMASK!(2, 0);
pub const PVR_TA_PM3_MIPMAP: u32                = BIT!(31);
pub const PVR_TA_PM3_TXRFMT: u32                = GENMASK!(30, 21);

pub const PVR_BINSIZE_0: c_int                  = 0;
pub const PVR_BINSIZE_8: c_int                  = 8;
pub const PVR_BINSIZE_16: c_int                 = 16;
pub const PVR_BINSIZE_32: c_int                 = 32;


#[repr(C, align(32))]
pub struct pvr_init_params_t {
    pub opb_sizes:              [c_int; 5],
    pub vertex_buf_size:        c_int,
    pub dma_enabled:            c_int,
    pub fsaa_enabled:           c_int,
    pub autosort_disabled:      c_int,
    pub opb_overflow_count:     c_int,
    pub vbuf_doublebuf_disabled: c_int,
}

#[repr(C)]
pub struct pvr_stats_t {
    pub frame_last_time:        u64,
    pub reg_last_time:          u64,
    pub rnd_last_time:          u64,
    pub buf_last_time:          u64,
    pub frame_count:            c_size_t,
    pub vbl_count:              c_size_t,
    pub vtx_buffer_used:        c_size_t,
    pub vtx_buffer_used_max:    c_size_t,
    pub frame_rate:             c_float,
    pub enabled_list_mask:      u32,
}

pub type pvr_dr_state_t = u32;

#[inline]
pub fn pvr_dr_target(state: &mut pvr_dr_state_t) -> *mut pvr_vertex_t {
    *state ^= 32;
    (crate::arch::memory::MEM_AREA_SQ_BASE | *state) as *mut pvr_vertex_t
}

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn pvr_init(params: *const pvr_init_params_t) -> c_int;
    pub fn pvr_init_defaults() -> c_int;
    pub fn pvr_shutdown() -> c_int;
    pub fn pvr_set_bg_color(r: c_float, g: c_float, b: c_float);
    pub fn pvr_set_shadow_scale(enable: bool, scale_value: c_float);
    pub fn pvr_set_zclip(zc: c_float);
    pub fn pvr_get_vbl_count() -> c_int;
    pub fn pvr_get_stats(stat: *mut pvr_stats_t) -> c_int;
    pub fn pvr_vertex_dma_enabled() -> c_int;
    pub fn pvr_set_vertbuf(list: pvr_list_t, buffer: *mut c_void, len: c_size_t) -> *mut c_void;
    pub fn pvr_vertbuf_tail(list: pvr_list_t) -> *mut c_void;
    pub fn pvr_vertbuf_written(list: pvr_list_t, amt: c_size_t);
    pub fn pvr_set_presort_mode(presort: bool);
    pub fn pvr_scene_begin();
    pub fn pvr_scene_begin_txr(txr: pvr_ptr_t, rx: *mut u32, ry: *mut u32);
    pub fn pvr_list_begin(list: pvr_list_t) -> c_int;
    pub fn pvr_list_finish() -> c_int;
    pub fn pvr_prim(data: *const c_void, size: c_size_t) -> c_int;
    pub fn pvr_dr_init(vtx_buf_ptr: *mut pvr_dr_state_t);
    #[link_name = "pvr_dr_commit_wrapper"]
    pub fn pvr_dr_commit(addr: *const c_void);
    pub fn pvr_dr_finish();
    pub fn pvr_send_to_ta(data: *mut c_void);
    pub fn pvr_list_prim(list: pvr_list_t, data: *const c_void, size: c_size_t) -> c_int;
    pub fn pvr_list_flush(list: pvr_list_t) -> c_int;
    pub fn pvr_scene_finish() -> c_int;
    pub fn pvr_wait_ready() -> c_int;
    pub fn pvr_check_ready() -> c_int;
    pub fn pvr_wait_render_done() -> c_int;
    pub fn pvr_poly_compile(dst: *mut pvr_poly_hdr_t, src: *const pvr_poly_cxt_t);
    pub fn pvr_poly_cxt_col(dst: *mut pvr_poly_cxt_t, list: pvr_list_t);
    pub fn pvr_poly_cxt_txr(
        dst: *mut pvr_poly_cxt_t,
        list: pvr_list_t,
        textureformat: c_int,
        tw: c_int,
        th: c_int,
        textureaddr: pvr_ptr_t,
        filtering: c_int,
    );
    pub fn pvr_sprite_compile(dst: *mut pvr_sprite_hdr_t, src: *const pvr_sprite_cxt_t);
    pub fn pvr_sprite_cxt_col(dst: *mut pvr_sprite_cxt_t, list: pvr_list_t);
    pub fn pvr_sprite_cxt_txr(
        dst: *mut pvr_sprite_cxt_t,
        list: pvr_list_t,
        textureformat: c_int,
        tw: c_int,
        th: c_int,
        textureaddr: pvr_ptr_t,
        filtering: c_int,
    );
    pub fn pvr_mod_compile(dst: *mut pvr_mod_hdr_t, list: pvr_list_t, mode: u32, cull: u32);
    pub fn pvr_poly_mod_compile(dst: *mut pvr_poly_mod_hdr_t, src: *const pvr_poly_cxt_t);
    pub fn pvr_poly_cxt_col_mod(dst: *mut pvr_poly_cxt_t, list: pvr_list_t);
    pub fn pvr_poly_cxt_txr_mod(
        dst: *mut pvr_poly_cxt_t,
        list: pvr_list_t,
        textureformat: c_int,
        tw: c_int,
        th: c_int,
        textureaddr: pvr_ptr_t,
        filtering: c_int,
        textureformat2: c_int,
        tw2: c_int,
        th2: c_int,
        textureaddr2: pvr_ptr_t,
        filtering2: c_int,
    );
    pub fn pvr_get_front_buffer() -> pvr_ptr_t;
}
