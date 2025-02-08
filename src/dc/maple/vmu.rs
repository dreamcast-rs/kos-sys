// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

use crate::BIT;

use super::maple_device_t;

pub const VMU_SCREEN_WIDTH: usize   = 48;
pub const VMU_SCREEN_HEIGHT: usize  = 32;

pub const VMU_DPAD_UP: u8           = BIT!(0);
pub const VMU_DPAD_DOWN: u8         = BIT!(1);
pub const VMU_DPAD_LEFT: u8         = BIT!(2);
pub const VMU_DPAD_RIGHT: u8        = BIT!(3);
pub const VMU_A: u8                 = BIT!(4);
pub const VMU_B: u8                 = BIT!(5);
pub const VMU_MODE: u8              = BIT!(6);
pub const VMU_SLEEP: u8             = BIT!(7);

#[repr(C)]
pub struct vmu_buttons_t {
    pub raw:            u8,
}

#[repr(C)]
pub struct buttons {
    pub current:        vmu_buttons_t,
    pub previous:       vmu_buttons_t,
}

#[repr(C)]
pub struct vmu_state_t {
    pub buttons:        buttons,
}

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn vmu_has_241_blocks(dev: *mut maple_device_t) -> c_int;
    pub fn vmu_toggle_241_blocks(dev: *mut maple_device_t) -> c_int;
    pub fn vmu_use_custom_color(dev: *mut maple_device_t, enable: c_int) -> c_int;
    pub fn vmu_set_custom_color(
        dev: *mut maple_device_t,
        red: u8,
        green: u8,
        red: u8,
        alpha: u8,
    ) -> c_int;
    pub fn vmu_get_custom_color(
        dev: *mut maple_device_t,
        red: *mut u8,
        green: *mut u8,
        red: *mut u8,
        alpha: *mut u8,
    ) -> c_int;
    pub fn vmu_set_icon_shape(dev: *mut maple_device_t, icon_shape: u8) -> c_int;
    pub fn vmu_get_icon_shape(dev: *mut maple_device_t, icon_shape: *mut u8) -> c_int;
    pub fn vmu_draw_lcd(dev: *mut maple_device_t, bitmap: *const c_void) -> c_int;
    pub fn vmu_draw_lcd_rotated(dev: *mut maple_device_t, bitmap: *const c_void) -> c_int;
    pub fn vmu_draw_lcd_xbm(dev: *mut maple_device_t, vmu_icon: *const c_char) -> c_int;
    pub fn vmu_set_icon(vmu_icon: *const c_char);
    pub fn vmu_block_read(dev: *mut maple_device_t, blocknum: u16, buffer: *mut u8) -> c_int;
    pub fn vmu_block_write(dev: *mut maple_device_t, blocknum: u16, buffer: *const u8) -> c_int;
    pub fn vmu_beep_raw(dev: *mut maple_device_t, beep: u32) -> c_int;
    pub fn vmu_beep_waveform(
        dev: *mut maple_device_t,
        period1: u8,
        duty_cycle1: u8,
        period2: u8,
        duty_cycle2: u8,
    ) -> c_int;
    pub fn vmu_set_datetime(dev: *mut maple_device_t, unix: time_t) -> c_int;
    pub fn vmu_get_datetime(dev: *mut maple_device_t, unix: *mut time_t) -> c_int;
    pub fn vmu_set_buttons_enabled(enable: c_int);
    pub fn vmu_get_buttons_enabled() -> c_int;
    pub fn vmu_init();
    pub fn vmu_shutdown();
}
