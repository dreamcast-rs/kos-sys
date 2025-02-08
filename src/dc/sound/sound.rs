// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn snd_mem_malloc(size: c_size_t) -> u32;
    pub fn snd_mem_free(addr: u32);
    pub fn snd_mem_available() -> u32;
    pub fn snd_mem_init(reserve: u32) -> c_int;
    pub fn snd_mem_shutdown();
    pub fn snd_init() -> c_int;
    pub fn snd_shutdown();
    pub fn snd_sh4_to_aica(packet: *mut c_void, size: u32) -> c_int;
    pub fn snd_sh4_to_aica_start();
    pub fn snd_sh4_to_aica_stop();
    pub fn snd_aia_to_sh4(packetout: *mut c_void) -> c_int;
    pub fn snd_poll_resp();
    pub fn snd_pcm16_split(data: *mut u32, left: *mut u32, right: *mut u32, size: c_size_t);
    pub fn snd_pcm16_split_sq(
        data: *mut u32,
        left: c_uintptr_t,
        right: c_uintptr_t,
        size: c_size_t,
    );
    pub fn snd_pcm8_split(data: *mut u32, left: *mut u32, right: *mut u32, size: c_size_t);
    pub fn snd_adpcm_split(data: *mut u32, left: *mut u32, right: *mut u32, size: c_size_t);
    pub fn snd_get_pos(ch: c_uint) -> u16;
    pub fn snd_is_playing(ch: c_uint) -> bool;
}
