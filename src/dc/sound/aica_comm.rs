// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[repr(C)]
pub struct aica_queue_t {
    pub head:       u32,
    pub tail:       u32,
    pub size:       u32,
    pub valid:      u32,
    pub process_ok: u32,
    pub data:       u32,
}

#[repr(C)]
pub struct aica_cmd_t {
    size:           u32,
    cmd:            u32,
    timestamp:      u32,
    cmd_id:         u32,
    misc:           [u32; 4],
    cmd_data:       FAM<u8>,
}

pub const AICA_CMD_MAX_SIZE: c_size_t   = 256;

#[repr(C)]
pub struct aica_channel_t {
    cmd:            u32,
    base:           u32,
    r#type:         u32,
    length:         u32,
    r#loop:         u32,
    loopstart:      u32,
    loopend:        u32,
    freq:           u32,
    vol:            u32,
    pan:            u32,
    pos:            u32,
    pad:            [u32; 5],
}

#[macro_export]
macro_rules! AICA_CMDSTR_CHANNEL {
    ($t:ident, $cmdr:ident, $chanr:ident) => {
        let mut $t = [0u32; core::mem::size_of::<aica_cmd_t>()
                          + core::mem::size_of::<aica_channel_t>() / 4];
        let $cmdr = $t.as_mut_ptr() as *mut aica_cmd_t;
        let $chanr = &mut (*$cmdr).cmd_data as *mut _ as *mut aica_channel_t;
    };
}

#[macro_export]
macro_rules! AICA_CMDSTR_CHANNEL_SIZE {
    () => {
        (core::mem::size_of::<aica_cmd_t>() + core::mem::size_of::<aica_channel_t>()) / 4
    }
}

pub const AICA_CMD_NONE: u32            = 0x00000000;
pub const AICA_CMD_PING: u32            = 0x00000001;
pub const AICA_CMD_CHAN: u32            = 0x00000002;
pub const AICA_CMD_SYNC_CLOCK: u32      = 0x00000003;

pub const AICA_RESP_NONE: u32           = 0x00000000;
pub const AICA_RESP_PONG: u32           = 0x00000001;
pub const AICA_RESP_DBGPRINT: u32       = 0x00000002;

pub const AICA_CH_CMD_MASK: u32         = 0x0000000F;

pub const AICA_CH_CMD_NONE: u32         = 0x00000000;
pub const AICA_CH_CMD_START: u32        = 0x00000001;
pub const AICA_CH_CMD_STOP: u32         = 0x00000002;
pub const AICA_CH_CMD_UPDATE: u32       = 0x00000003;


pub const AICA_CH_START_MASK: u32       = 0x00300000;

pub const AICA_CH_START_DELAY: u32      = 0x00100000;
pub const AICA_CH_START_SYNC: u32       = 0x00200000;

pub const AICA_CH_UPDATE_MASK: u32      = 0x000FF000;

pub const AICA_CH_UPDATE_SET_FREQ: u32  = 0x00001000;
pub const AICA_CH_UPDATE_SET_VOL: u32   = 0x00002000;
pub const AICA_CH_UPDATE_SET_PAN: u32   = 0x00004000;

pub const AICA_SM_16BIT: u32            = 0;
pub const AICA_SM_8BIT: u32             = 1;
pub const AICA_SM_ADPCM: u32            = 2;
pub const AICA_SM_ADPCM_LS: u32         = 3;
