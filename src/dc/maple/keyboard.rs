// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;
use super::maple_device_t;

pub const KBD_MOD_LCTRL: u8             = 1 << 0;
pub const KBD_MOD_LSHIFT: u8            = 1 << 1;
pub const KBD_MOD_LALT: u8              = 1 << 2;
pub const KBD_MOD_S1: u8                = 1 << 3;
pub const KBD_MOD_RCTRL: u8             = 1 << 4;
pub const KBD_MOD_RSHIFT: u8            = 1 << 5;
pub const KBD_MOD_RALT: u8              = 1 << 6;
pub const KBD_MOD_S2: u8                = 1 << 7;

pub const KBD_LED_NUMLOCK: u8           = 1 << 0;
pub const KBD_LED_CAPSLOCK: u8          = 1 << 1;
pub const KBD_LED_SCRLOCK: u8           = 1 << 2;

pub const KBD_KEY_NONE: u8              = 0x00;
pub const KBD_KEY_ERROR: u8             = 0x01;
pub const KBD_KEY_ERR2: u8              = 0x02;
pub const KBD_KEY_ERR3: u8              = 0x03;
pub const KBD_KEY_A: u8                 = 0x04;
pub const KBD_KEY_B: u8                 = 0x05;
pub const KBD_KEY_C: u8                 = 0x06;
pub const KBD_KEY_D: u8                 = 0x07;
pub const KBD_KEY_E: u8                 = 0x08;
pub const KBD_KEY_F: u8                 = 0x09;
pub const KBD_KEY_G: u8                 = 0x0A;
pub const KBD_KEY_H: u8                 = 0x0B;
pub const KBD_KEY_I: u8                 = 0x0C;
pub const KBD_KEY_J: u8                 = 0x0D;
pub const KBD_KEY_K: u8                 = 0x0E;
pub const KBD_KEY_L: u8                 = 0x0F;
pub const KBD_KEY_M: u8                 = 0x10;
pub const KBD_KEY_N: u8                 = 0x11;
pub const KBD_KEY_O: u8                 = 0x12;
pub const KBD_KEY_P: u8                 = 0x13;
pub const KBD_KEY_Q: u8                 = 0x14;
pub const KBD_KEY_R: u8                 = 0x15;
pub const KBD_KEY_S: u8                 = 0x16;
pub const KBD_KEY_T: u8                 = 0x17;
pub const KBD_KEY_U: u8                 = 0x18;
pub const KBD_KEY_V: u8                 = 0x19;
pub const KBD_KEY_W: u8                 = 0x1A;
pub const KBD_KEY_X: u8                 = 0x1B;
pub const KBD_KEY_Y: u8                 = 0x1C;
pub const KBD_KEY_Z: u8                 = 0x1D;
pub const KBD_KEY_1: u8                 = 0x1E;
pub const KBD_KEY_2: u8                 = 0x1F;
pub const KBD_KEY_3: u8                 = 0x20;
pub const KBD_KEY_4: u8                 = 0x21;
pub const KBD_KEY_5: u8                 = 0x22;
pub const KBD_KEY_6: u8                 = 0x23;
pub const KBD_KEY_7: u8                 = 0x24;
pub const KBD_KEY_8: u8                 = 0x25;
pub const KBD_KEY_9: u8                 = 0x26;
pub const KBD_KEY_0: u8                 = 0x27;
pub const KBD_KEY_ENTER: u8             = 0x28;
pub const KBD_KEY_ESCAPE: u8            = 0x29;
pub const KBD_KEY_BACKSPACE: u8         = 0x2A;
pub const KBD_KEY_TAB: u8               = 0x2B;
pub const KBD_KEY_SPACE: u8             = 0x2C;
pub const KBD_KEY_MINUS: u8             = 0x2D;
pub const KBD_KEY_PLUS: u8              = 0x2E;
pub const KBD_KEY_LBRACKET: u8          = 0x2F;
pub const KBD_KEY_RBRACKET: u8          = 0x30;
pub const KBD_KEY_BACKSLASH: u8         = 0x31;
pub const KBD_KEY_SEMICOLON: u8         = 0x33;
pub const KBD_KEY_QUOTE: u8             = 0x34;
pub const KBD_KEY_TILDE: u8             = 0x35;
pub const KBD_KEY_COMMA: u8             = 0x36;
pub const KBD_KEY_PERIOD: u8            = 0x37;
pub const KBD_KEY_SLASH: u8             = 0x38;
pub const KBD_KEY_CAPSLOCK: u8          = 0x39;
pub const KBD_KEY_F1: u8                = 0x3A;
pub const KBD_KEY_F2: u8                = 0x3B;
pub const KBD_KEY_F3: u8                = 0x3C;
pub const KBD_KEY_F4: u8                = 0x3D;
pub const KBD_KEY_F5: u8                = 0x3E;
pub const KBD_KEY_F6: u8                = 0x3F;
pub const KBD_KEY_F7: u8                = 0x40;
pub const KBD_KEY_F8: u8                = 0x41;
pub const KBD_KEY_F9: u8                = 0x42;
pub const KBD_KEY_F10: u8               = 0x43;
pub const KBD_KEY_F11: u8               = 0x44;
pub const KBD_KEY_F12: u8               = 0x45;
pub const KBD_KEY_PRINT: u8             = 0x46;
pub const KBD_KEY_SCRLOCK: u8           = 0x47;
pub const KBD_KEY_PAUSE: u8             = 0x48;
pub const KBD_KEY_INSERT: u8            = 0x49;
pub const KBD_KEY_HOME: u8              = 0x4A;
pub const KBD_KEY_PGUP: u8              = 0x4B;
pub const KBD_KEY_DEL: u8               = 0x4C;
pub const KBD_KEY_END: u8               = 0x4D;
pub const KBD_KEY_PGDOWN: u8            = 0x4E;
pub const KBD_KEY_RIGHT: u8             = 0x4F;
pub const KBD_KEY_LEFT: u8              = 0x50;
pub const KBD_KEY_DOWN: u8              = 0x51;
pub const KBD_KEY_UP: u8                = 0x52;
pub const KBD_KEY_PAD_NUMLOCK: u8       = 0x53;
pub const KBD_KEY_PAD_DIVIDE: u8        = 0x54;
pub const KBD_KEY_PAD_MULTIPLY: u8      = 0x55;
pub const KBD_KEY_PAD_MINUS: u8         = 0x56;
pub const KBD_KEY_PAD_PLUS: u8          = 0x57;
pub const KBD_KEY_PAD_ENTER: u8         = 0x58;
pub const KBD_KEY_PAD_1: u8             = 0x59;
pub const KBD_KEY_PAD_2: u8             = 0x5A;
pub const KBD_KEY_PAD_3: u8             = 0x5B;
pub const KBD_KEY_PAD_4: u8             = 0x5C;
pub const KBD_KEY_PAD_5: u8             = 0x5D;
pub const KBD_KEY_PAD_6: u8             = 0x5E;
pub const KBD_KEY_PAD_7: u8             = 0x5F;
pub const KBD_KEY_PAD_8: u8             = 0x60;
pub const KBD_KEY_PAD_9: u8             = 0x61;
pub const KBD_KEY_PAD_0: u8             = 0x62;
pub const KBD_KEY_PAD_PERIOD: u8        = 0x63;
pub const KBD_KEY_S3: u8                = 0x65;

pub const KBD_REGION_JP: c_int          = 1;
pub const KBD_REGION_US: c_int          = 2;
pub const KBD_REGION_UK: c_int          = 3;
pub const KBD_REGION_DE: c_int          = 4;
pub const KBD_REGION_FR: c_int          = 5;
pub const KBD_REGION_IT: c_int          = 6;
pub const KBD_REGION_ES: c_int          = 7;

pub const KEY_STATE_NONE: u8            = 0;
pub const KEY_STATE_WAS_PRESSED: u8     = 1;
pub const KEY_STATE_PRESSED: u8         = 2;

pub const MAX_PRESSED_KEYS: c_size_t    = 6;
pub const MAX_KBD_KEYS: c_size_t        = 256;
pub const KBD_QUEUE_SIZE: c_size_t      = 16;

#[repr(C)]
pub struct kbd_keymap_t {
    pub base:               [u8; MAX_KBD_KEYS],
    pub shifted:            [u8; MAX_KBD_KEYS],
    pub alt:                [u8; MAX_KBD_KEYS],
}

#[repr(C)]
pub struct kbd_cond_t {
    pub modifiers:          u8,
    pub leds:               u8,
    pub keys:               [u8; MAX_PRESSED_KEYS],
}

#[repr(C)]
pub struct kbd_state_t {
    pub cond:               kbd_cond_t,
    pub matrix:             [u8; MAX_KBD_KEYS],
    pub shift_keys:         c_int,
    pub region:             c_int,
    pub key_queue:          [u32; KBD_QUEUE_SIZE],
    pub queue_tail:         c_int,
    pub queue_head:         c_int,
    pub queue_len:          c_int, // volatile
    pub kbd_repeat_key:     u8,
    pub kbd_repeat_timer:   u64,
}

#[link(name = "kallisti")]
extern "C" {
    pub fn kbd_set_queue(active: c_int);
    pub fn kbd_get_key() -> c_int;
    pub fn kbd_queue_pop(dev: *mut maple_device_t, xlat: c_int) -> c_int;
    pub fn kbd_init();
    pub fn kbd_shutdown();
}
