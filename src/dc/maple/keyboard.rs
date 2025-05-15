// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;
use crate::BIT;

use super::maple_device_t;

pub const KBD_MOD_LCTRL: u8             = BIT!(0);
pub const KBD_MOD_LSHIFT: u8            = BIT!(1);
pub const KBD_MOD_LALT: u8              = BIT!(2);
pub const KBD_MOD_S1: u8                = BIT!(3);
pub const KBD_MOD_RCTRL: u8             = BIT!(4);
pub const KBD_MOD_RSHIFT: u8            = BIT!(5);
pub const KBD_MOD_RALT: u8              = BIT!(6);
pub const KBD_MOD_S2: u8                = BIT!(7);

pub const KBD_MOD_CTRL: u8              = KBD_MOD_LCTRL | KBD_MOD_RCTRL;
pub const KBD_MOD_SHIFT: u8             = KBD_MOD_LSHIFT | KBD_MOD_RSHIFT;
pub const KBD_MOD_ALT: u8               = KBD_MOD_LALT | KBD_MOD_RALT;

#[repr(C)]
pub struct kbd_mods_t {
    pub raw: u8,
}

pub const KBD_LED_NUMLOCK: u8           = BIT!(0);
pub const KBD_LED_CAPSLOCK: u8          = BIT!(1);
pub const KBD_LED_SCRLOCK: u8           = BIT!(2);
pub const KBD_LED_UNKNOWN1: u8          = BIT!(3);
pub const KBD_LED_UNKNOWN2: u8          = BIT!(4);
pub const KBD_LED_KANA: u8              = BIT!(5);
pub const KBD_LED_POWER: u8             = BIT!(6);
pub const KBD_LED_SHIFT: u8             = BIT!(7);

#[repr(C)]
pub struct kbd_leds_t {
    pub raw: u8,
}

#[repr(C)]
pub enum kbd_region_t {
    KBD_REGION_JP = 1,
    KBD_REGION_US = 2,
    KBD_REGION_UK = 3,
    KBD_REGION_DE = 4,
    KBD_REGION_FR = 5,
    KBD_REGION_IT = 6,
    KBD_REGION_ES = 7,
}

#[repr(i8)]
pub enum kbd_key_t {
    KBD_KEY_NONE            = 0x00,
    KBD_KEY_ERROR           = 0x01,
    KBD_KEY_ERR2            = 0x02,
    KBD_KEY_ERR3            = 0x03,
    KBD_KEY_A               = 0x04,
    KBD_KEY_B               = 0x05,
    KBD_KEY_C               = 0x06,
    KBD_KEY_D               = 0x07,
    KBD_KEY_E               = 0x08,
    KBD_KEY_F               = 0x09,
    KBD_KEY_G               = 0x0A,
    KBD_KEY_H               = 0x0B,
    KBD_KEY_I               = 0x0C,
    KBD_KEY_J               = 0x0D,
    KBD_KEY_K               = 0x0E,
    KBD_KEY_L               = 0x0F,
    KBD_KEY_M               = 0x10,
    KBD_KEY_N               = 0x11,
    KBD_KEY_O               = 0x12,
    KBD_KEY_P               = 0x13,
    KBD_KEY_Q               = 0x14,
    KBD_KEY_R               = 0x15,
    KBD_KEY_S               = 0x16,
    KBD_KEY_T               = 0x17,
    KBD_KEY_U               = 0x18,
    KBD_KEY_V               = 0x19,
    KBD_KEY_W               = 0x1A,
    KBD_KEY_X               = 0x1B,
    KBD_KEY_Y               = 0x1C,
    KBD_KEY_Z               = 0x1D,
    KBD_KEY_1               = 0x1E,
    KBD_KEY_2               = 0x1F,
    KBD_KEY_3               = 0x20,
    KBD_KEY_4               = 0x21,
    KBD_KEY_5               = 0x22,
    KBD_KEY_6               = 0x23,
    KBD_KEY_7               = 0x24,
    KBD_KEY_8               = 0x25,
    KBD_KEY_9               = 0x26,
    KBD_KEY_0               = 0x27,
    KBD_KEY_ENTER           = 0x28,
    KBD_KEY_ESCAPE          = 0x29,
    KBD_KEY_BACKSPACE       = 0x2A,
    KBD_KEY_TAB             = 0x2B,
    KBD_KEY_SPACE           = 0x2C,
    KBD_KEY_MINUS           = 0x2D,
    KBD_KEY_PLUS            = 0x2E,
    KBD_KEY_LBRACKET        = 0x2F,
    KBD_KEY_RBRACKET        = 0x30,
    KBD_KEY_BACKSLASH       = 0x31,
    KBD_KEY_SEMICOLON       = 0x33,
    KBD_KEY_QUOTE           = 0x34,
    KBD_KEY_TILDE           = 0x35,
    KBD_KEY_COMMA           = 0x36,
    KBD_KEY_PERIOD          = 0x37,
    KBD_KEY_SLASH           = 0x38,
    KBD_KEY_CAPSLOCK        = 0x39,
    KBD_KEY_F1              = 0x3A,
    KBD_KEY_F2              = 0x3B,
    KBD_KEY_F3              = 0x3C,
    KBD_KEY_F4              = 0x3D,
    KBD_KEY_F5              = 0x3E,
    KBD_KEY_F6              = 0x3F,
    KBD_KEY_F7              = 0x40,
    KBD_KEY_F8              = 0x41,
    KBD_KEY_F9              = 0x42,
    KBD_KEY_F10             = 0x43,
    KBD_KEY_F11             = 0x44,
    KBD_KEY_F12             = 0x45,
    KBD_KEY_PRINT           = 0x46,
    KBD_KEY_SCRLOCK         = 0x47,
    KBD_KEY_PAUSE           = 0x48,
    KBD_KEY_INSERT          = 0x49,
    KBD_KEY_HOME            = 0x4A,
    KBD_KEY_PGUP            = 0x4B,
    KBD_KEY_DEL             = 0x4C,
    KBD_KEY_END             = 0x4D,
    KBD_KEY_PGDOWN          = 0x4E,
    KBD_KEY_RIGHT           = 0x4F,
    KBD_KEY_LEFT            = 0x50,
    KBD_KEY_DOWN            = 0x51,
    KBD_KEY_UP              = 0x52,
    KBD_KEY_PAD_NUMLOCK     = 0x53,
    KBD_KEY_PAD_DIVIDE      = 0x54,
    KBD_KEY_PAD_MULTIPLY    = 0x55,
    KBD_KEY_PAD_MINUS       = 0x56,
    KBD_KEY_PAD_PLUS        = 0x57,
    KBD_KEY_PAD_ENTER       = 0x58,
    KBD_KEY_PAD_1           = 0x59,
    KBD_KEY_PAD_2           = 0x5A,
    KBD_KEY_PAD_3           = 0x5B,
    KBD_KEY_PAD_4           = 0x5C,
    KBD_KEY_PAD_5           = 0x5D,
    KBD_KEY_PAD_6           = 0x5E,
    KBD_KEY_PAD_7           = 0x5F,
    KBD_KEY_PAD_8           = 0x60,
    KBD_KEY_PAD_9           = 0x61,
    KBD_KEY_PAD_0           = 0x62,
    KBD_KEY_PAD_PERIOD      = 0x63,
    KBD_KEY_S3              = 0x65,
}

pub const KEY_STATE_NONE: u8            = 0;
pub const KEY_STATE_WAS_PRESSED: u8     = 1;
pub const KEY_STATE_PRESSED: u8         = 2;

pub const MAX_PRESSED_KEYS: c_size_t    = 6;
pub const KBD_MAX_KEYS: c_size_t        = 256;

#[repr(C)]
pub struct kbd_cond_t {
    pub modifiers:          kbd_mods_t,
    pub leds:               kbd_leds_t,
    pub keys:               [kbd_key_t; MAX_PRESSED_KEYS],
}

#[repr(C)]
pub struct kbd_state_t {
    pub cond:               kbd_cond_t,
    pub matrix:             [u8; KBD_MAX_KEYS],
    pub last_modifiers:     kbd_mods_t,
    pub region:             kbd_region_t,
}

pub const KBD_QUEUE_SIZE: c_size_t      = 16;
pub const KBD_QUEUE_END: c_int          = -1;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn kbd_key_to_ascii(key: kbd_key_t, region: kbd_region_t,
                            mods: kbd_mods_t, leds: kbd_leds_t) -> c_char;
    pub fn kbd_get_state(device: *mut maple_device_t) -> *mut kbd_state_t;
    pub fn kbd_set_repeat_timing(start: u16, interval: u16);
    pub fn kbd_queue_pop(dev: *mut maple_device_t, xlat: bool) -> c_int;
    pub fn kbd_init();
    pub fn kbd_shutdown();
}
