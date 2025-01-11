// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;
use libc::time_t;

#[link(name = "kallisti")]
extern "C" {
    pub fn rtc_unix_secs() -> time_t;
    pub fn rtc_set_unix_secs(time: time_t) -> c_int;
    pub fn rtc_boot_time() -> time_t;
    pub fn rtc_init() -> c_int;
    pub fn rtc_shutdown();
}
