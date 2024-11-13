#![allow(non_camel_case_types)]

use crate::prelude::*;

pub type sfxhnd_t = u32;

pub const SFXHND_INVALID: sfxhnd_t  = 0;

extern "C" {
    pub fn snd_sfx_load(r#fn: *const c_char) -> sfxhnd_t;
    pub fn snd_sfx_unload(idx: sfxhnd_t);
    pub fn snd_sfx_unload_all();
    pub fn snd_sfx_play(idx: sfxhnd_t, vol: c_int, pan: c_int) -> c_int;
    pub fn snd_sfx_play_chn(chn: c_int, sdx: sfxhnd_t, vol: c_int, pan: c_int) -> c_int;
    pub fn snd_sfx_stop(chn: c_int);
    pub fn snd_sfx_stop_all();
    pub fn snd_sfx_chn_alloc() -> c_int;
    pub fn snd_sfx_chn_free(chn: c_int);
}