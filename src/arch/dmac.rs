// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub type dma_callback_t = Option<unsafe extern "C" fn(data: *mut c_void)>;

#[repr(C)]
pub enum dma_channel_t {
    DMA_CHANNEL_0,
    DMA_CHANNEL_1,
    DMA_CHANNEL_2,
    DMA_CHANNEL_3,
}

#[repr(C)]
pub enum dma_request_t {
    DMA_REQUEST_EXTERNAL_MEM_TO_MEM = 0,
    DMA_REQUEST_EXTERNAL_MEM_TO_DEV = 2,
    DMA_REQUEST_EXTERNAL_DEV_TO_MEM = 3,
    DMA_REQUEST_AUTO_MEM_TO_MEM     = 4,
    DMA_REQUEST_AUTO_MEM_TO_DEV     = 5,
    DMA_REQUEST_AUTO_DEV_TO_MEM     = 6,
    DMA_REQUEST_SCI_TRANSMIT        = 8,
    DMA_REQUEST_SCI_RECEIVE         = 9,
    DMA_REQUEST_SCIF_TRANSMIT       = 10,
    DMA_REQUEST_SCIF_RECEIVE        = 11,
    DMA_REQUEST_TMU2_MEM_TO_MEM     = 12,
    DMA_REQUEST_TMU2_MEM_TO_DEV     = 13,
    DMA_REQUEST_TMU2_DEV_TO_MEM     = 14,
}

#[repr(C)]
pub enum dma_unitsize_t {
    DMA_UNITSIZE_64BIT,
    DMA_UNITSIZE_8BIT,
    DMA_UNITSIZE_16BIT,
    DMA_UNITSIZE_32BIT,
    DMA_UNITSIZE_32BYTE,
}

#[repr(C)]
pub enum dma_addrmode_t {
    DMA_ADDRMODE_FIXED,
    DMA_ADDRMODE_INCREMENT,
    DMA_ADDRMODE_DECREMENT,
}

#[repr(C)]
pub enum dma_transmitmode_t {
    DMA_TRANSMITMODE_CYCLE_STEAL,
    DMA_TRANSMITMODE_BURST,
}

#[repr(C)]
pub struct dma_config_t {
    channel:        dma_channel_t,
    request:        dma_request_t,
    unit_size:      dma_unitsize_t,
    src_mode:       dma_addrmode_t,
    dst_mode:       dma_addrmode_t,
    transmit_mode:  dma_transmitmode_t,
    callback:       dma_callback_t,
}

pub type dma_addr_t = u32;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn hw_to_dma_addr(hw_addr: c_uintptr_t) -> dma_addr_t;
    pub fn dma_map_src(ptr: *const c_void, len: c_size_t) -> dma_addr_t;
    pub fn dma_map_dst(ptr: *mut c_void, len: c_size_t) -> dma_addr_t;
    pub fn dma_transfer(
        cfg: *const dma_config_t,
        dst: dma_addr_t,
        src: dma_addr_t,
        len: c_size_t,
        cb_data: *mut c_void,
    ) -> c_int;
    pub fn dma_wait_complete(channel: dma_channel_t);
    pub fn dma_transfer_get_remaining(channel: dma_channel_t) -> c_size_t;
}
