// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[repr(C)]
pub enum WDT_CLK_DIV {
    WDT_CLK_DIV_32,
    WDT_CLK_DIV_64,
    WDT_CLK_DIV_128,
    WDT_CLK_DIV_256,
    WDT_CLK_DIV_512,
    WDT_CLK_DIV_1024,
    WDT_CLK_DIV_2048,
    WDT_CLK_DIV_4096,
}

#[repr(C)]
pub enum WDT_RST {
    WDT_RST_POWER_ON,
    WDT_RST_MANUAL,
}

pub type wdt_callback = Option<unsafe extern "C" fn(user_data: *mut c_void)>;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn wdt_enable_timer(initial_count: u8, microsec_period: u32, irq_prio: u8,
                            callback: wdt_callback, user_data: *mut c_void);
    pub fn wdt_enable_watchdog(initial_count: u8, clk_config: WDT_CLK_DIV,
                               reset_select: WDT_RST);
    pub fn wdt_get_counter() -> u8;
    pub fn wdt_set_counter(value: u8);
    pub fn wdt_pet();
    pub fn wdt_disable();
    pub fn wdt_is_enabled() -> c_int;
}
