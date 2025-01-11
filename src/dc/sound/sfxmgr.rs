// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

use crate::os::fs::file_t;
use crate::prelude::*;

pub type sfxhnd_t = u32;

pub const SFXHND_INVALID: sfxhnd_t  = 0;

#[link(name = "kallisti")]
extern "C" {
    pub fn snd_sfx_load(r#fn: *const c_char) -> sfxhnd_t;
    pub fn snd_sfx_load_ex(r#fn: *const c_char, rate: u32, bitsize: u16,
                           channels: u16) -> sfxhnd_t;
    pub fn snd_sfx_load_fd(fd: file_t, len: c_size_t, rate: u32, bitsize: u16,
                           channels: u16) -> sfxhnd_t;
    pub fn snd_sfx_load_buf(buf: *mut c_char) -> sfxhnd_t;
    pub fn snd_sfx_load_raw_buf(buf: *mut c_char, len: c_size_t, rate: u32,
                                bitsize: u16, channels: u16) -> sfxhnd_t;
    pub fn snd_sfx_unload(idx: sfxhnd_t);
    pub fn snd_sfx_unload_all();
    pub fn snd_sfx_play(idx: sfxhnd_t, vol: c_int, pan: c_int) -> c_int;
    pub fn snd_sfx_play_chn(chn: c_int, sdx: sfxhnd_t, vol: c_int, pan: c_int) -> c_int;
    pub fn snd_sfx_stop(chn: c_int);
    pub fn snd_sfx_stop_all();
    pub fn snd_sfx_chn_alloc() -> c_int;
    pub fn snd_sfx_chn_free(chn: c_int);
}
