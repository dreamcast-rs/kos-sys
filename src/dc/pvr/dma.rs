// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

use super::*;

#[repr(C)]
pub enum pvr_dma_type_t {
    PVR_DMA_VRAM64,
    PVR_DMA_VRAM32,
    PVR_DMA_TA,
    PVR_DMA_YUV,
    PVR_DMA_VRAM32_SB,
    PVR_DMA_VRAM64_SB,
}

pub type pvr_dma_callback_t = Option<unsafe extern "C" fn(data: *mut c_void)>;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn pvr_dma_transfer(
        src: *const c_void,
        dest: c_uintptr_t,
        count: c_size_t,
        r#type: pvr_dma_type_t,
        block: bool,
        callback: pvr_dma_callback_t,
        cbdata: *mut c_void,
    ) -> c_int;
    pub fn pvr_txr_load_dma(
        src: *const c_void,
        dest: pvr_ptr_t,
        count: c_size_t,
        block: bool,
        callback: pvr_dma_callback_t,
        cbdata: *mut c_void,
    ) -> c_int;
    pub fn pvr_dma_load_ta(
        src: *const c_void,
        count: c_size_t,
        block: bool,
        callback: pvr_dma_callback_t,
        cbdata: *mut c_void,
    ) -> c_int;
    pub fn pvr_dma_yuv_conv(
        src: *const c_void,
        count: c_size_t,
        block: bool,
        callback: pvr_dma_callback_t,
        cbdata: *mut c_void,
    ) -> c_int;
    pub fn pvr_dma_ready() -> bool;
    pub fn pvr_dma_init();
    pub fn pvr_dma_shutdown();
    pub fn pvr_sq_load(
        dest: *mut c_void,
        src: *const c_void,
        n: c_size_t,
        r#type: pvr_dma_type_t,
    ) -> *mut c_void;
    pub fn pvr_sq_set16(
        dest: *mut c_void,
        c: u32,
        n: c_size_t,
        r#type: pvr_dma_type_t,
    ) -> *mut c_void;
    pub fn pvr_sq_set32(
        dest: *mut c_void,
        c: u32,
        n: c_size_t,
        r#type: pvr_dma_type_t,
    ) -> *mut c_void;
}
