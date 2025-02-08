// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

use super::*;

pub const PVR_TXRLOAD_4BPP: u32                 = 0x01;
pub const PVR_TXRLOAD_8BPP: u32                 = 0x02;
pub const PVR_TXRLOAD_16BPP: u32                = 0x03;
pub const PVR_TXRLOAD_FMT_MASK: u32             = 0x0F;

pub const PVR_TXRLOAD_VQ_LOAD: u32              = 0x10;
pub const PVR_TXRLOAD_INVERT_Y: u32             = 0x20;
pub const PVR_TXRLOAD_FMT_VQ: u32               = 0x40;
pub const PVR_TXRLOAD_FMT_TWIDDLED: u32         = 0x80;
pub const PVR_TXRLOAD_FMT_NOTWIDDLE: u32        = 0x80;
pub const PVR_TXRLOAD_DMA: u32                  = 0x8000;
pub const PVR_TXRLOAD_NONBLOCK: u32             = 0x4000;
pub const PVR_TXRLOAD_SQ: u32                   = 0x2000;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn pvr_txr_load(src: *const c_void, dst: pvr_ptr_t, count: u32);
    pub fn pvr_txr_load_ex(src: *const c_void, dst: pvr_ptr_t, w: u32, h: u32, flags: u32);
    pub fn pvr_txr_load_kimg(img: *const crate::addons::img::kos_img_t, dst: pvr_ptr_t, flags: u32);
}
