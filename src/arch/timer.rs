// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub const TMU0: c_int           = 0;
pub const TMU1: c_int           = 1;
pub const TMU2: c_int           = 2;

// 👻 Prevent the Curse of the Timer Unit! 👻
pub const TIMER_UNIT_0: c_int   = TMU0;
pub const TIMER_UNIT_1: c_int   = TMU1;
pub const TIMER_UNIT_2: c_int   = TMU2;

pub const TIMER_ID: c_int       = TMU0;

pub type timer_primary_callback_t = Option<unsafe extern "C"
                                          fn(*mut super::irq::irq_context_t)>;

#[link(name = "kallisti")]
extern "C" {
    pub fn timer_prime(channel: c_int, speed: u32, interrupts: c_int) -> c_int;
    pub fn timer_start(channel: c_int) -> c_int;
    pub fn timer_stop(channel: c_int) -> c_int;
    pub fn timer_running(channel: c_int) -> c_int;
    pub fn timer_count(channel: c_int) -> u32;
    pub fn timer_clear(channel: c_int) -> c_int;
    pub fn timer_enable_ints(channel: c_int);
    pub fn timer_disable_ints(channel: c_int);
    pub fn timer_ints_enabled(channel: c_int) -> c_int;
    pub fn timer_ms_enable();
    pub fn timer_ms_disable();
    pub fn timer_ms_gettime(secs: *mut u32, msecs: *mut u32);
    pub fn timer_ms_gettime64() -> u64;
    pub fn timer_us_gettime(secs: *mut u32, usecs: *mut u32);
    pub fn timer_us_gettime64() -> u64;
    pub fn timer_ns_gettime(secs: *mut u32, nsecs: *mut u32);
    pub fn timer_ns_gettime64() -> u64;
    pub fn timer_spin_sleep(ms: c_int);
    pub fn timer_spin_delay_us(us: c_ushort);
    pub fn timer_spin_delay_ns(ns: c_ushort);
    pub fn timer_primary_set_callback(callback: timer_primary_callback_t)
                                      -> timer_primary_callback_t;
    pub fn timer_primary_weakeup(millis: u32);
    pub fn timer_init() -> c_int;
    pub fn timer_shutdown();
}
