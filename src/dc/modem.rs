// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub const MODEM_SPEED_AUTO: c_uchar         = 0x0;
pub const MODEM_SPEED_1200: c_uchar         = 0x0;
pub const MODEM_SPEED_2400: c_uchar         = 0x1;
pub const MODEM_SPEED_4800: c_uchar         = 0x2;
pub const MODEM_SPEED_7200: c_uchar         = 0x3;
pub const MODEM_SPEED_9600: c_uchar         = 0x4;
pub const MODEM_SPEED_12000: c_uchar        = 0x5;
pub const MODEM_SPEED_14400: c_uchar        = 0x6;
pub const MODEM_SPEED_16800: c_uchar        = 0x7;
pub const MODEM_SPEED_19200: c_uchar        = 0x8;
pub const MODEM_SPEED_21600: c_uchar        = 0x9;
pub const MODEM_SPEED_24000: c_uchar        = 0xA;
pub const MODEM_SPEED_26400: c_uchar        = 0xB;
pub const MODEM_SPEED_28000: c_uchar        = 0xC;
pub const MODEM_SPEED_31200: c_uchar        = 0xD;
pub const MODEM_SPEED_33600: c_uchar        = 0xE;

pub const MODEM_PROTOCOL_V17: c_uchar       = 0x0;
pub const MODEM_PROTOCOL_V22: c_uchar       = 0x1;
pub const MODEM_PROTOCOL_V22BIS: c_uchar    = 0x2;
pub const MODEM_PROTOCOL_V32: c_uchar       = 0x3;
pub const MODEM_PROTOCOL_V32BIS: c_uchar    = 0x4;
pub const MODEM_PROTOCOL_V34: c_uchar       = 0x5;
pub const MODEM_PROTOCOL_V8: c_uchar        = 0x6;

#[inline]
pub const fn MODEM_SPEED_GET_PROTOCOL(x: modem_speed_t) -> c_uchar {
    x >> 4
}

#[inline]
pub const fn MODEM_SPEED_GET_SPEED(x: modem_speed_t) -> c_uchar {
    x & 0xF
}

#[inline]
pub const fn MODEM_MAKE_SPEED(p: c_uchar, s: c_uchar) -> modem_speed_t {
    ((p & 0xF) << 4) | (s & 0xF)
}

pub type modem_speed_t = c_uchar;

pub const MODEM_MODE_REMOTE: c_int          = 0;
pub const MODEM_MODE_ANSWER: c_int          = 1;
pub const MODEM_MODE_NULL: c_int            = 255;

pub const MODEM_SPEED_V22BIS_1200: modem_speed_t  = MODEM_MAKE_SPEED(MODEM_PROTOCOL_V22BIS, MODEM_SPEED_1200);
pub const MODEM_SPEED_V22BIS_2400: modem_speed_t  = MODEM_MAKE_SPEED(MODEM_PROTOCOL_V22BIS, MODEM_SPEED_2400);
pub const MODEM_SPEED_V22_1200: modem_speed_t     = MODEM_MAKE_SPEED(MODEM_PROTOCOL_V22, MODEM_SPEED_1200);
pub const MODEM_SPEED_V32_4800: modem_speed_t     = MODEM_MAKE_SPEED(MODEM_PROTOCOL_V32, MODEM_SPEED_4800);
pub const MODEM_SPEED_V32_9600: modem_speed_t     = MODEM_MAKE_SPEED(MODEM_PROTOCOL_V32, MODEM_SPEED_9600);
pub const MODEM_SPEED_V32BIS_7200: modem_speed_t  = MODEM_MAKE_SPEED(MODEM_PROTOCOL_V32BIS, MODEM_SPEED_7200);
pub const MODEM_SPEED_V32BIS_12000: modem_speed_t = MODEM_MAKE_SPEED(MODEM_PROTOCOL_V32BIS, MODEM_SPEED_12000);
pub const MODEM_SPEED_V32BIS_14400: modem_speed_t = MODEM_MAKE_SPEED(MODEM_PROTOCOL_V32BIS, MODEM_SPEED_14400);
pub const MODEM_SPEED_V8_2400: modem_speed_t      = MODEM_MAKE_SPEED(MODEM_PROTOCOL_V8, MODEM_SPEED_2400);
pub const MODEM_SPEED_V8_4800: modem_speed_t      = MODEM_MAKE_SPEED(MODEM_PROTOCOL_V8, MODEM_SPEED_4800);
pub const MODEM_SPEED_V8_7200: modem_speed_t      = MODEM_MAKE_SPEED(MODEM_PROTOCOL_V8, MODEM_SPEED_7200);
pub const MODEM_SPEED_V8_9600: modem_speed_t      = MODEM_MAKE_SPEED(MODEM_PROTOCOL_V8, MODEM_SPEED_9600);
pub const MODEM_SPEED_V8_12000: modem_speed_t     = MODEM_MAKE_SPEED(MODEM_PROTOCOL_V8, MODEM_SPEED_12000);
pub const MODEM_SPEED_V8_14400: modem_speed_t     = MODEM_MAKE_SPEED(MODEM_PROTOCOL_V8, MODEM_SPEED_14400);
pub const MODEM_SPEED_V8_16800: modem_speed_t     = MODEM_MAKE_SPEED(MODEM_PROTOCOL_V8, MODEM_SPEED_16800);
pub const MODEM_SPEED_V8_19200: modem_speed_t     = MODEM_MAKE_SPEED(MODEM_PROTOCOL_V8, MODEM_SPEED_19200);
pub const MODEM_SPEED_V8_21600: modem_speed_t     = MODEM_MAKE_SPEED(MODEM_PROTOCOL_V8, MODEM_SPEED_21600);
pub const MODEM_SPEED_V8_24000: modem_speed_t     = MODEM_MAKE_SPEED(MODEM_PROTOCOL_V8, MODEM_SPEED_24000);
pub const MODEM_SPEED_V8_26400: modem_speed_t     = MODEM_MAKE_SPEED(MODEM_PROTOCOL_V8, MODEM_SPEED_26400);
pub const MODEM_SPEED_V8_28000: modem_speed_t     = MODEM_MAKE_SPEED(MODEM_PROTOCOL_V8, MODEM_SPEED_28000);
pub const MODEM_SPEED_V8_31200: modem_speed_t     = MODEM_MAKE_SPEED(MODEM_PROTOCOL_V8, MODEM_SPEED_31200);
pub const MODEM_SPEED_V8_33600: modem_speed_t     = MODEM_MAKE_SPEED(MODEM_PROTOCOL_V8, MODEM_SPEED_33600);
pub const MODEM_SPEED_V8_AUTO: modem_speed_t      = MODEM_MAKE_SPEED(MODEM_PROTOCOL_V8, MODEM_SPEED_1200);

#[repr(C)]
pub enum modemEvent_t {
    MODEM_EVENT_CONNECTION_FAILED = 0,
    MODEM_EVENT_CONNECTED,
    MODEM_EVENT_DISCONNECTED,
    MODEM_EVENT_RX_NOT_EMPTY,
    MODEM_EVENT_OVERFLOW,
    MODEM_EVENT_TX_EMPTY
}

pub type MODEMEVENTHANDLERPROC = Option<unsafe extern "C" fn(event: modemEvent_t)>;

#[link(name = "kallisti")]
extern "C" {
    pub fn modem_init() -> c_int;
    pub fn modem_shutdown();
    pub fn modem_set_mode(mode: c_int, speed: modem_speed_t) -> c_int;
    pub fn modem_wait_dialtone(ms_timeout: c_int) -> c_int;
    pub fn modem_dial(digits: *const c_char) -> c_int;
    pub fn modem_set_event_handler(eventHandler: MODEMEVENTHANDLERPROC);
    pub fn modem_disconnect();
    pub fn modem_is_connecting() -> c_int;
    pub fn modem_is_connected() -> c_int;
    pub fn modem_get_connection_rate() -> c_ulong;
    pub fn modem_read_data(data: *mut c_uchar, size: c_int) -> c_int;
    pub fn modem_write_data(data: *mut c_uchar, size: c_int) -> c_int;
    pub fn modem_has_data() -> c_int;
}
