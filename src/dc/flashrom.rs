// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub const FLASHROM_PT_SYSTEM: c_int         = 0;
pub const FLASHROM_PT_RESERVED: c_int       = 1;
pub const FLASHROM_PT_BLOCK_1: c_int        = 2;
pub const FLASHROM_PT_SETTINGS: c_int       = 3;
pub const FLASHROM_PT_BLOCK_2: c_int        = 4;

pub const FLASHROM_B1_SYSCFG: c_int         = 0x05;
pub const FLASHROM_B1_PW_SETTINGS_1: c_int  = 0x80;
pub const FLASHROM_B1_PW_SETTINGS_2: c_int  = 0x81;
pub const FLASHROM_B1_PW_SETTINGS_3: c_int  = 0x82;
pub const FLASHROM_B1_PW_SETTINGS_4: c_int  = 0x83;
pub const FLASHROM_B1_PW_SETTINGS_5: c_int  = 0x84;
pub const FLASHROM_B1_PW_PPP1: c_int        = 0xC0;
pub const FLASHROM_B1_PW_PPP2: c_int        = 0xC1;
pub const FLASHROM_B1_PW_DNS: c_int         = 0xC2;
pub const FLASHROM_B1_PW_EMAIL1: c_int      = 0xC3;
pub const FLASHROM_B1_PW_EMAIL2: c_int      = 0xC4;
pub const FLASHROM_B1_PW_EMAIL_PROXY: c_int = 0xC5;
pub const FLASHROM_B1_DK_PPP1: c_int        = 0xC6;
pub const FLASHROM_B1_DK_PPP2: c_int        = 0xC7;
pub const FLASHROM_B1_DK_DNS: c_int         = 0xC8;
pub const FLASHROM_B1_IP_SETTINGS: c_int    = 0xE0;
pub const FLASHROM_B1_EMAIL: c_int          = 0xE2;
pub const FLASHROM_B1_SMTP: c_int           = 0xE4;
pub const FLASHROM_B1_POP3: c_int           = 0xE5;
pub const FLASHROM_B1_POP3LOGIN: c_int      = 0xE6;
pub const FLASHROM_B1_POP3PASSWD: c_int     = 0xE7;
pub const FLASHROM_B1_PPPLOGIN: c_int       = 0xE8;
pub const FLASHROM_B1_PPPPASSWD: c_int      = 0xE9;
pub const FLASHROM_B1_PPPMODEM: c_int       = 0xEB;

pub const FLASHROM_OFFSET_CRC: c_int        = 62;

pub const FLASHROM_ERR_NONE: c_int          = 0;
pub const FLASHROM_ERR_NOT_FOUND: c_int     = -1;
pub const FLASHROM_ERR_NO_PARTITION: c_int  = -2;
pub const FLASHROM_ERR_READ_PART: c_int     = -3;
pub const FLASHROM_ERR_BAD_MAGIC: c_int     = -4;
pub const FLASHROM_ERR_BOGUS_PART: c_int    = -5;
pub const FLASHROM_ERR_NOMEM: c_int         = -6;
pub const FLASHROM_ERR_READ_BITMAP: c_int   = -7;
pub const FLASHROM_ERR_EMPTY_PART: c_int    = -8;
pub const FLASHROM_ERR_READ_BLOCK: c_int    = -9;

pub const FLASHROM_LANG_JAPANESE: c_int     = 0;
pub const FLASHROM_LANG_ENGLISH: c_int      = 1;
pub const FLASHROM_LANG_GERMAN: c_int       = 2;
pub const FLASHROM_LANG_FRENCH: c_int       = 3;
pub const FLASHROM_LANG_SPANISH: c_int      = 4;
pub const FLASHROM_LANG_ITALIAN: c_int      = 5;

#[repr(C)]
pub struct flashrom_syscfg_t {
    language:       c_int,
    audio:          c_int,
    autostart:      c_int,
}

pub const FLASHROM_REGION_UNKNOWN: c_int    = 0;
pub const FLASHROM_REGION_JAPAN: c_int      = 1;
pub const FLASHROM_REGION_US: c_int         = 2;
pub const FLASHROM_REGION_EUROPE: c_int     = 3;

pub const FLASHROM_ISP_DIALUP: c_int        = 0;
pub const FLASHROM_ISP_DHCP: c_int          = 1;
pub const FLASHROM_ISP_PPPOE: c_int         = 2;
pub const FLASHROM_ISP_STATIC: c_int        = 3;

pub const FLASHROM_ISP_IP: u32              = 1 <<  0;
pub const FLASHROM_ISP_NETMASK: u32         = 1 <<  1;
pub const FLASHROM_ISP_BROADCAST: u32       = 1 <<  2;
pub const FLASHROM_ISP_GATEWAY: u32         = 1 <<  3;
pub const FLASHROM_ISP_DNS: u32             = 1 <<  4;
pub const FLASHROM_ISP_HOSTNAME: u32        = 1 <<  5;
pub const FLASHROM_ISP_EMAIL: u32           = 1 <<  6;
pub const FLASHROM_ISP_SMTP: u32            = 1 <<  7;
pub const FLASHROM_ISP_POP3: u32            = 1 <<  8;
pub const FLASHROM_ISP_POP3_USER: u32       = 1 <<  9;
pub const FLASHROM_ISP_POP3_PASS: u32       = 1 << 10;
pub const FLASHROM_ISP_PROXY_HOST: u32      = 1 << 11;
pub const FLASHROM_ISP_PROXY_PORT: u32      = 1 << 12;
pub const FLASHROM_ISP_PPP_USER: u32        = 1 << 13;
pub const FLASHROM_ISP_PPP_PASS: u32        = 1 << 14;
pub const FLASHROM_ISP_OUT_PREFIX: u32      = 1 << 15;
pub const FLASHROM_ISP_CW_PREFIX: u32       = 1 << 16;
pub const FLASHROM_ISP_REAL_NAME: u32       = 1 << 17;
pub const FLASHROM_ISP_MODEM_INIT: u32      = 1 << 18;
pub const FLASHROM_ISP_AREA_CODE: u32       = 1 << 19;
pub const FLASHROM_ISP_LD_PREFIX: u32       = 1 << 20;
pub const FLASHROM_ISP_PHONE1: u32          = 1 << 21;
pub const FLASHROM_ISP_PHONE2: u32          = 1 << 22;

pub const FLASHROM_ISP_DIAL_AREACODE: u32   = 1 <<  0;
pub const FLASHROM_ISP_USE_PROXY: u32       = 1 <<  1;
pub const FLASHROM_ISP_PULSE_DIAL: u32      = 1 <<  2;
pub const FLASHROM_ISP_BLIND_DIAL: u32      = 1 <<  3;

#[repr(C)]
pub struct flashrom_ispcfg_t {
    method:         c_int,
    valid_fields:   u32,
    flags:          u32,
    ip:             [u8; 4],
    nm:             [u8; 4],
    bc:             [u8; 4],
    gw:             [u8; 4],
    dns:            [[u8; 4]; 2],
    proxy_port:     c_int,
    hostname:       [c_char; 24],
    email:          [c_char; 64],
    smtp:           [c_char; 31],
    pop3:           [c_char; 31],
    pop3_login:     [c_char; 20],
    pop3_passwd:    [c_char; 32],
    proxy_host:     [c_char; 31],
    ppp_login:      [c_char; 29],
    ppp_passwd:     [c_char; 20],
    out_prefix:     [c_char; 9],
    cw_prefix:      [c_char; 9],
    real_name:      [c_char; 31],
    modem_init:     [c_char; 33],
    area_code:      [c_char; 4],
    ld_prefix:      [c_char; 21],
    p1_areacode:    [c_char; 4],
    phone1:         [c_char; 26],
    p2_areacode:    [c_char; 4],
    phone2:         [c_char; 26],
}

#[link(name = "kallisti")]
extern "C" {
    pub fn flashrom_info(part: c_int, start_out: *mut c_int,
                         size_out: *mut c_int) -> c_int;
    pub fn flashrom_read(offset: c_int, buffer_out: *mut c_void, bytes: c_int) -> c_int;
    pub fn flashrom_write(offset: c_int, buffer: *mut c_void, bytes: c_int) -> c_int;
    pub fn flashrom_delete(offset: c_int) -> c_int;
    pub fn flashrom_get_block(partid: c_int, blockid: c_int,
                              buffer_out: *mut u8) -> c_int;
    pub fn flashrom_get_syscfg(out: *mut flashrom_syscfg_t) -> c_int;
    pub fn flashrom_get_region() -> c_int;
    pub fn flashrom_get_ispcfg(out: *mut flashrom_ispcfg_t) -> c_int;
    pub fn flashrom_get_pw_ispcfg(out: *mut flashrom_ispcfg_t) -> c_int;
}
