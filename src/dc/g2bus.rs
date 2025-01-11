// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub const G2_DMA_CHAN_SPU: u32  = 0;
pub const G2_DMA_CHAN_BBA: u32  = 1;
pub const G2_DMA_CHAN_CH2: u32  = 2;
pub const G2_DMA_CHAN_CH3: u32  = 3;

pub const G2_DMA_TO_G2: u32     = 0;
pub const G2_DMA_TO_SH4: u32    = 1;

pub type g2_dma_callback_t = Option<unsafe extern "C" fn(data: *mut c_void)>;

#[repr(C)]
pub struct g2_ctx_t {
    irq_state:  c_int,
}

#[link(name = "kallisti")]
extern "C" {
    pub fn g2_dma_transfer(sh4: *mut c_void, g2bus: *mut c_void, length: c_size_t,
                           block: u32, callback: g2_dma_callback_t, cbdata: *mut c_void,
                           dir: u32, mode: u32, g2chn: u32, sh4chn: u32) -> c_int;
    pub fn g2_dma_init() -> c_int;
    pub fn g2_dma_shutdown();
    #[link_name = "g2_lock_wrapper"]
    pub fn g2_lock() -> g2_ctx_t;
    #[link_name = "g2_unlock_wrapper"]
    pub fn g2_unlock(ctx: g2_ctx_t);
    pub fn g2_read_8(address: c_uintptr_t) -> u8;
    pub fn g2_write_8(address: c_uintptr_t, value: u8);
    pub fn g2_read_16(address: c_uintptr_t) -> u16;
    pub fn g2_write_16(address: c_uintptr_t, value: u16);
    pub fn g2_read_32(address: c_uintptr_t) -> u32;
    pub fn g2_write_32(address: c_uintptr_t, value: u32);
    pub fn g2_read_block_8(output: *mut u8, address: c_uintptr_t, amt: c_size_t);
    pub fn g2_write_block_8(input: *const u8, address: c_uintptr_t, amt: c_size_t);
    pub fn g2_read_block_16(output: *mut u16, address: c_uintptr_t, amt: c_size_t);
    pub fn g2_write_block_16(input: *const u16, address: c_uintptr_t, amt: c_size_t);
    pub fn g2_read_block_32(output: *mut u32, address: c_uintptr_t, amt: c_size_t);
    pub fn g2_write_block_32(input: *const u32, address: c_uintptr_t, amt: c_size_t);
    pub fn g2_memset_8(address: c_uintptr_t, c: u8, amt: c_size_t);
    pub fn g2_fifo_wait();
}
