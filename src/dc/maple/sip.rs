// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;
use super::maple_device_t;

pub type sip_sample_cb = Option<unsafe extern "C" fn(dev: *mut maple_device_t,
                                                     samples: *mut u8, len: c_size_t)>;

#[repr(C)]
pub struct sip_state_t {
    pub amp_gain:       c_int,
    pub sample_type:    c_int,
    pub frequency:      c_int,
    pub is_sampling:    c_int,
    pub callback:       sip_sample_cb,
}

pub const SIP_SUBCOMMAND_GET_SAMPLES: c_int = 0x01;
pub const SIP_SUBCOMMAND_BASIC_CTRL: c_int  = 0x02;

pub const SIP_MIN_GAIN: c_uint              = 0x00;
pub const SIP_DEFAULT_GAIN: c_uint          = 0x0F;
pub const SIP_MAX_GAIN: c_uint              = 0x1F;

pub const SIP_SAMPLE_16BIT_SIGNED: c_uint   = 0x00;
pub const SIP_SAMPLE_8BIT_ULAW: c_uint      = 0x01;

pub const SIP_SAMPLE_11KHZ: c_uint          = 0x00;
pub const SIP_SAMPLE_8KHZ: c_uint           = 0x01;

#[link(name = "kallisti")]
extern "C" {
    pub fn sip_set_gain(dev: *mut maple_device_t, g: c_uint) -> c_int;
    pub fn sip_set_sample_type(dev: *mut maple_device_t, r#type: c_uint) -> c_int;
    pub fn sip_set_frequency(dev: *mut maple_device_t, freq: c_uint) -> c_int;
    pub fn sip_start_sampling(dev: *mut maple_device_t, cb: sip_sample_cb,
                              block: c_int) -> c_int;
    pub fn sip_stop_sampling(dev: *mut maple_device_t, block: c_int) -> c_int;
    pub fn sip_init();
    pub fn sip_shutdown();
}
