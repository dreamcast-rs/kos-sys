// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;
use super::maple_device_t;

#[repr(C)]
pub struct purupuru_effect_t {
    pub duration:   u8,
    pub effect2:    u8,
    pub effect1:    u8,
    pub special:    u8,
}

#[macro_export]
macro_rules! PURUPURU_EFFECT2_UINTENSITY {
    ($x:expr) => {
        $x << 4
    };
}
#[macro_export]
macro_rules! PURUPURU_EFFECT2_LINTENSITY {
    ($x:expr) => {
        $x
    };
}
pub const PURUPURU_EFFECT2_DECAY: u8        = 8 << 4;
pub const PURUPURU_EFFECT2_PULSE: u8        = 8;
#[macro_export]
macro_rules! PURUPURU_EFFECT1_INTENSITY {
    ($x:expr) => {
        $x << 4
    };
}
pub const PURUPURU_EFFECT1_PULSE: u8        = 8 << 4;
pub const PURUPURU_EFFECT1_POWERSAVE: u8    = 15;
pub const PURUPURU_SPECIAL_MOTOR1: u8       = 1 << 4;
pub const PURUPURU_SPECIAL_MOTOR2: u8       = 1 << 7;
pub const PURUPURU_SPECIAL_PULSE: u8        = 1;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn purupuru_rumble(dev: *mut maple_device_t, effect: *mut purupuru_effect_t) -> c_int;
    pub fn purupuru_rumble_raw(dev: *mut maple_device_t, effect: u32) -> c_int;
    pub fn purupuru_init();
    pub fn purupuru_shutdown();
}
