// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn rtc_unix_secs() -> time_t;
    pub fn rtc_set_unix_secs(time: time_t) -> c_int;
    pub fn rtc_boot_time() -> time_t;
    pub fn rtc_init() -> c_int;
    pub fn rtc_shutdown();
}
