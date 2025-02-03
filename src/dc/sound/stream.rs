// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub const SND_STREAM_MAX: c_size_t              = 4;
pub const SND_STREAM_BUFFER_MAX_PCM16: c_size_t = 128 << 10;
pub const SND_STREAM_BUFFER_MAX_PCM8: c_size_t  = 64 << 10;
pub const SND_STREAM_BUFFER_MAX_ADPCM: c_size_t = 32 << 10;
pub const SND_STREAM_BUFFER_MAX: c_size_t       = 64 << 10;

pub type snd_stream_hnd_t = c_int;

pub const SND_STREAM_INVALID: snd_stream_hnd_t  = -1;

pub type snd_stream_callback_t = Option<unsafe extern "C" fn(hnd: snd_stream_hnd_t,
                                                             smp_req: c_int,
                                                             smp_recv: *mut c_int)
                                                             -> *mut c_void>;

pub type snd_stream_callback_direct_t = Option<unsafe extern "C" fn(hnd: snd_stream_hnd_t,
                                                                    left: c_uintptr_t,
                                                                    right: c_uintptr_t,
                                                                    size_req: c_size_t)
                                                                    -> c_size_t>;

pub type snd_stream_filter_t = Option<unsafe extern "C" fn(hnd: snd_stream_hnd_t,
                                                           obj: *mut c_void, hz: c_int,
                                                           channels: c_int,
                                                           buffer: *mut *mut c_void,
                                                           samplecnt: *mut c_int)>;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn snd_stream_set_callback(hnd: snd_stream_hnd_t, cb: snd_stream_callback_t);
    pub fn snd_stream_set_callback_direct(hnd: snd_stream_hnd_t,
                                          cb: snd_stream_callback_direct_t);
    pub fn snd_stream_set_userdata(hnd: snd_stream_hnd_t, d: *mut c_void);
    pub fn snd_stream_get_userdata(hnd: snd_stream_hnd_t) -> *mut c_void;
    pub fn snd_stream_filter_add(hnd: snd_stream_hnd_t, filtfunc: snd_stream_filter_t,
                                 obj: *mut c_void);
    pub fn snd_stream_filter_remove(hnd: snd_stream_hnd_t, filtfunc: snd_stream_filter_t,
                                    obj: *mut c_void);
    pub fn snd_stream_prefill(hnd: snd_stream_hnd_t);
    pub fn snd_stream_init() -> c_int;
    pub fn snd_stream_init_ex(channels: c_int, buffer_size: c_size_t) -> c_int;
    pub fn snd_stream_shutdown();
    pub fn snd_stream_alloc(cb: snd_stream_callback_t, bufsize: c_int)
                            -> snd_stream_hnd_t;
    pub fn snd_stream_reinit(hnd: snd_stream_hnd_t, cb: snd_stream_callback_t) -> c_int;
    pub fn snd_stream_destroy(hnd: snd_stream_hnd_t);
    pub fn snd_stream_queue_enable(hnd: snd_stream_hnd_t);
    pub fn snd_stream_queue_disable(hnd: snd_stream_hnd_t);
    pub fn snd_stream_queue_go(hnd: snd_stream_hnd_t);
    pub fn snd_stream_start(hnd: snd_stream_hnd_t, freq: u32, st: c_int);
    pub fn snd_stream_start_pcm8(hnd: snd_stream_hnd_t, freq: u32, st: c_int);
    pub fn snd_stream_start_adpcm(hnd: snd_stream_hnd_t, freq: u32, st: c_int);
    pub fn snd_stream_stop(hnd: snd_stream_hnd_t);
    pub fn snd_stream_poll(hnd: snd_stream_hnd_t) -> c_int;
    pub fn snd_stream_volume(hnd: snd_stream_hnd_t, vol: c_int);
    pub fn snd_stream_pan(hnd: snd_stream_hnd_t, left_pan: c_int, right_pan: c_int);
}
