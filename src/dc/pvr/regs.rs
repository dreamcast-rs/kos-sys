// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[macro_export]
macro_rules! PVR_GET {
    ($reg:expr) => {
        *((0xA05F8000 as *const u32).offset($reg as isize))
    };
}

#[macro_export]
macro_rules! PVR_SET {
    ($reg:expr, $value:expr) => {
        *((0xA05F8000 as *mut u32).offset($reg as isize)) = $value;
    };
}

pub const PVR_ID: u32                           = 0x0000;
pub const PVR_REVISION: u32                     = 0x0004;
pub const PVR_RESET: u32                        = 0x0008;

pub const PVR_ISP_START: u32                    = 0x0014;
pub const PVR_UNK_0018: u32                     = 0x0018;

pub const PVR_ISP_VERTBUF_ADDR: u32             = 0x0020;

pub const PVR_ISP_TILEMAT_ADDR: u32             = 0x002C;
pub const PVR_SPANSORT_CFG: u32                 = 0x0030;

pub const PVR_BORDER_COLOR: u32                 = 0x0040;
pub const PVR_FB_CFG_1: u32                     = 0x0044;
pub const PVR_FB_CFG_2: u32                     = 0x0048;
pub const PVR_RENDER_MODULO: u32                = 0x004C;
pub const PVR_FB_ADDR: u32                      = 0x0050;
pub const PVR_FB_IL_ADDR: u32                   = 0x0054;

pub const PVR_FB_SIZE: u32                      = 0x005C;
pub const PVR_RENDER_ADDR: u32                  = 0x0060;
pub const PVR_RENDER_ADDR_2: u32                = 0x0064;
pub const PVR_PCLIP_X: u32                      = 0x0068;
pub const PVR_PCLIP_Y: u32                      = 0x006C;

pub const PVR_CHEAP_SHADOW: u32                 = 0x0074;
pub const PVR_OBJECT_CLIP: u32                  = 0x0078;
pub const PVR_UNK_007C: u32                     = 0x007C;
pub const PVR_UNK_0080: u32                     = 0x0080;
pub const PVR_TEXTURE_CLIP: u32                 = 0x0084;
pub const PVR_BGPLANE_Z: u32                    = 0x0088;
pub const PVR_BGPLANE_CFG: u32                  = 0x008C;

pub const PVR_UNK_0098: u32                     = 0x0098;

pub const PVR_UNK_00A0: u32                     = 0x00A0;

pub const PVR_UNK_00A8: u32                     = 0x00A8;

pub const PVR_FOG_TABLE_COLOR: u32              = 0x00B0;
pub const PVR_FOG_VERTEX_COLOR: u32             = 0x00B4;
pub const PVR_FOG_DENSITY: u32                  = 0x00B8;
pub const PVR_COLOR_CLAMP_MAX: u32              = 0x00Bc;
pub const PVR_COLOR_CLAMP_MIN: u32              = 0x00C0;
pub const PVR_GUN_POS: u32                      = 0x00C4;
pub const PVR_HPOS_IRQ: u32                     = 0x00C8;
pub const PVR_VPOS_IRQ: u32                     = 0x00CC;
pub const PVR_IL_CFG: u32                       = 0x00D0;
pub const PVR_BORDER_X: u32                     = 0x00D4;
pub const PVR_SCAN_CLK: u32                     = 0x00D8;
pub const PVR_BORDER_Y: u32                     = 0x00DC;

pub const PVR_TEXTURE_MODULO: u32               = 0x00E4;
pub const PVR_VIDEO_CFG: u32                    = 0x00E8;
pub const PVR_BITMAP_X: u32                     = 0x00EC;
pub const PVR_BITMAP_Y: u32                     = 0x00F0;
pub const PVR_SCALER_CFG: u32                   = 0x00F4;

pub const PVR_PALETTE_CFG: u32                  = 0x0108;
pub const PVR_SYNC_STATUS: u32                  = 0x010C;
pub const PVR_UNK_0110: u32                     = 0x0110;
pub const PVR_UNK_0114: u32                     = 0x0114;
pub const PVR_UNK_0118: u32                     = 0x0118;

pub const PVR_TA_OPB_START: u32                 = 0x0124;
pub const PVR_TA_VERTBUF_START: u32             = 0x0128;
pub const PVR_TA_OPB_END: u32                   = 0x012C;
pub const PVR_TA_VERTBUF_END: u32               = 0x0130;
pub const PVR_TA_OPB_POS: u32                   = 0x0134;
pub const PVR_TA_VERTBUF_POS: u32               = 0x0138;
pub const PVR_TILEMAT_CFG: u32                  = 0x013C;
pub const PVR_OPB_CFG: u32                      = 0x0140;
pub const PVR_TA_INIT: u32                      = 0x0144;
pub const PVR_YUV_ADDR: u32                     = 0x0148;
pub const PVR_YUV_CFG: u32                      = 0x014C;
pub const PVR_YUV_STAT: u32                     = 0x0150;

pub const PVR_UNK_0160: u32                     = 0x0160;
pub const PVR_TA_OPB_INIT: u32                  = 0x0164;

pub const PVR_FOG_TABLE_BASE: u32               = 0x0200;

pub const PVR_PALETTE_TABLE_BASE: u32           = 0x1000;

pub const PVR_TA_INPUT: c_uintptr_t             = 0x10000000;
pub const PVR_TA_YUV_CONV: c_uintptr_t          = 0x10800000;
pub const PVR_TA_TEX_MEM: c_uintptr_t           = 0x11000000;
pub const PVR_TA_TEX_MEM_32: c_uintptr_t        = 0x13000000;
pub const PVR_RAM_BASE_32_P0: c_uintptr_t       = 0x05000000;
pub const PVR_RAM_BASE_64_P0: c_uintptr_t       = 0x04000000;
pub const PVR_RAM_BASE: c_uintptr_t             = 0xA5000000;
pub const PVR_RAM_INT_BASE: c_uintptr_t         = 0xA4000000;

pub const PVR_RAM_SIZE: c_size_t                = 8*1024*1024;

pub const PVR_RAM_TOP: c_uintptr_t              = PVR_RAM_BASE + PVR_RAM_SIZE;
pub const PVR_RAM_INT_TOP: c_uintptr_t          = PVR_RAM_INT_BASE + PVR_RAM_SIZE;

pub const PVR_RESET_ALL: u32                    = 0xFFFFFFFF;
pub const PVR_RESET_NONE: u32                   = 0x00000000;
pub const PVR_RESET_TA: u32                     = 0x00000001;
pub const PVR_RESET_ISPTSP: u32                 = 0x00000002;

pub const PVR_ISP_START_GO: u32                 = 0xFFFFFFFF;

pub const PVR_TA_INIT_GO: u32                   = 0x80000000;
