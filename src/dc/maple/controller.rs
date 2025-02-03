// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;
use super::maple_device_t;

pub const CONT_C: u32                               = 1 << 0;
pub const CONT_B: u32                               = 1 << 1;
pub const CONT_A: u32                               = 1 << 2;
pub const CONT_START: u32                           = 1 << 3;
pub const CONT_DPAD_UP: u32                         = 1 << 4;
pub const CONT_DPAD_DOWN: u32                       = 1 << 5;
pub const CONT_DPAD_LEFT: u32                       = 1 << 6;
pub const CONT_DPAD_RIGHT: u32                      = 1 << 7;
pub const CONT_Z: u32                               = 1 << 8;
pub const CONT_Y: u32                               = 1 << 9;
pub const CONT_X: u32                               = 1 << 10;
pub const CONT_D: u32                               = 1 << 11;
pub const CONT_DPAD2_UP: u32                        = 1 << 12;
pub const CONT_DPAD2_DOWN: u32                      = 1 << 13;
pub const CONT_DPAD2_LEFT: u32                      = 1 << 14;
pub const CONT_DPAD2_RIGHT: u32                     = 1 << 15;
pub const CONT_RESET_BUTTONS: u32                   = CONT_A | CONT_B | CONT_X | CONT_Y |
                                                      CONT_START;

#[repr(C)]
pub struct cont_state_t {
    pub buttons: u32,
    pub ltrig: c_int,
    pub rtrig: c_int,
    pub joyx: c_int,
    pub joyy: c_int,
    pub joy2x: c_int,
    pub joy2y: c_int,
}

pub type cont_btn_callback_t = Option<unsafe extern "C" fn(addr: u8, btns: u32)>;

pub const CONT_CAPABILITY_C: u32                    = 1 << 24;
pub const CONT_CAPABILITY_B: u32                    = 1 << 25;
pub const CONT_CAPABILITY_A: u32                    = 1 << 26;
pub const CONT_CAPABILITY_START: u32                = 1 << 27;
pub const CONT_CAPABILITY_DPAD_UP: u32              = 1 << 28;
pub const CONT_CAPABILITY_DPAD_DOWN: u32            = 1 << 29;
pub const CONT_CAPABILITY_DPAD_LEFT: u32            = 1 << 30;
pub const CONT_CAPABILITY_DPAD_RIGHT: u32           = 1 << 31;
pub const CONT_CAPABILITY_Z: u32                    = 1 << 16;
pub const CONT_CAPABILITY_Y: u32                    = 1 << 17;
pub const CONT_CAPABILITY_X: u32                    = 1 << 18;
pub const CONT_CAPABILITY_D: u32                    = 1 << 19;
pub const CONT_CAPABILITY_DPAD2_UP: u32             = 1 << 20;
pub const CONT_CAPABILITY_DPAD2_DOWN: u32           = 1 << 21;
pub const CONT_CAPABILITY_DPAD2_LEFT: u32           = 1 << 22;
pub const CONT_CAPABILITY_DPAD2_RIGHT: u32          = 1 << 23;
pub const CONT_CAPABILITY_RTRIG: u32                = 1 << 8;
pub const CONT_CAPABILITY_LTRIG: u32                = 1 << 9;
pub const CONT_CAPABILITY_ANALOG_X: u32             = 1 << 10;
pub const CONT_CAPABILITY_ANALOG_Y: u32             = 1 << 11;
pub const CONT_CAPABILITY_ANALOG2_X: u32            = 1 << 12;
pub const CONT_CAPABILITY_ANALOG2_Y: u32            = 1 << 13;

pub const CONT_CAPABILITIES_STANDARD_BUTTONS: u32   = CONT_CAPABILITY_A |
                                                      CONT_CAPABILITY_B |
                                                      CONT_CAPABILITY_X |
                                                      CONT_CAPABILITY_Y |
                                                      CONT_CAPABILITY_START;
pub const CONT_CAPABILITIES_DPAD: u32               = CONT_CAPABILITY_DPAD_UP |
                                                      CONT_CAPABILITY_DPAD_DOWN |
                                                      CONT_CAPABILITY_DPAD_LEFT |
                                                      CONT_CAPABILITY_DPAD_RIGHT;
pub const CONT_CAPABILITIES_ANALOG: u32             = CONT_CAPABILITY_ANALOG_X |
                                                      CONT_CAPABILITY_ANALOG_Y;
pub const CONT_CAPABILITIES_TRIGGERS: u32           = CONT_CAPABILITY_LTRIG |
                                                      CONT_CAPABILITY_RTRIG;
pub const CONT_CAPABILITIES_EXTENDED_BUTTONS: u32   = CONT_CAPABILITY_C |
                                                      CONT_CAPABILITY_Z;
pub const CONT_CAPABILITIES_SECONDARY_DPAD: u32     = CONT_CAPABILITY_DPAD2_UP |
                                                      CONT_CAPABILITY_DPAD2_DOWN |
                                                      CONT_CAPABILITY_DPAD2_LEFT |
                                                      CONT_CAPABILITY_DPAD2_RIGHT;
pub const CONT_CAPABILITIES_SECONDARY_ANALOG: u32   = CONT_CAPABILITY_ANALOG2_X |
                                                      CONT_CAPABILITY_ANALOG2_Y;
pub const CONT_CAPABILITIES_DUAL_DPAD: u32          = CONT_CAPABILITIES_DPAD |
                                                      CONT_CAPABILITIES_SECONDARY_DPAD;
pub const CONT_CAPABILITIES_DUAL_ANALOG: u32        = CONT_CAPABILITIES_ANALOG |
                                                      CONT_CAPABILITIES_SECONDARY_ANALOG;

pub const CONT_TYPE_STANDARD_CONTROLLER: u32        = CONT_CAPABILITIES_STANDARD_BUTTONS |
                                                      CONT_CAPABILITIES_TRIGGERS |
                                                      CONT_CAPABILITIES_DPAD |
                                                      CONT_CAPABILITIES_ANALOG;
pub const CONT_TYPE_DUAL_ANALOG_CONTROLLER: u32     = CONT_CAPABILITIES_STANDARD_BUTTONS |
                                                      CONT_CAPABILITIES_TRIGGERS |
                                                      CONT_CAPABILITIES_DPAD |
                                                      CONT_CAPABILITIES_DUAL_ANALOG;
pub const CONT_TYPE_ASCII_PAD: u32                  = CONT_CAPABILITIES_STANDARD_BUTTONS |
                                                      CONT_CAPABILITIES_EXTENDED_BUTTONS |
                                                      CONT_CAPABILITIES_DPAD;
pub const CONT_TYPE_ARCADE_STICK: u32               = CONT_CAPABILITIES_STANDARD_BUTTONS |
                                                      CONT_CAPABILITIES_EXTENDED_BUTTONS |
                                                      CONT_CAPABILITIES_DPAD;
pub const CONT_TYPE_TWIN_STICK: u32                 = CONT_CAPABILITIES_STANDARD_BUTTONS |
                                                      CONT_CAPABILITIES_EXTENDED_BUTTONS |
                                                      CONT_CAPABILITY_D |
                                                      CONT_CAPABILITIES_DUAL_DPAD;
pub const CONT_TYPE_ASCII_MISSION_STICK: u32        = CONT_CAPABILITIES_STANDARD_BUTTONS |
                                                      CONT_CAPABILITIES_DUAL_DPAD |
                                                      CONT_CAPABILITIES_TRIGGERS |
                                                      CONT_CAPABILITIES_ANALOG;
pub const CONT_TYPE_RACING_CONTROLLER: u32          = CONT_CAPABILITY_DPAD_UP |
                                                      CONT_CAPABILITY_DPAD_DOWN |
                                                      CONT_CAPABILITY_A |
                                                      CONT_CAPABILITY_B |
                                                      CONT_CAPABILITY_START |
                                                      CONT_CAPABILITIES_TRIGGERS |
                                                      CONT_CAPABILITY_ANALOG_X |
                                                      CONT_CAPABILITIES_SECONDARY_ANALOG;
pub const CONT_TYPE_MARACAS: u32                    = CONT_CAPABILITY_A |
                                                      CONT_CAPABILITY_B |
                                                      CONT_CAPABILITY_D |
                                                      CONT_CAPABILITY_START |
                                                      CONT_CAPABILITIES_EXTENDED_BUTTONS |
                                                      CONT_CAPABILITIES_DUAL_ANALOG;
pub const CONT_TYPE_DANCE_MAT: u32                  = CONT_CAPABILITY_A |
                                                      CONT_CAPABILITY_B |
                                                      CONT_CAPABILITY_START |
                                                      CONT_CAPABILITIES_DPAD;
pub const CONT_TYPE_FISHING_ROD: u32                = CONT_CAPABILITIES_STANDARD_BUTTONS |
                                                      CONT_CAPABILITIES_DPAD |
                                                      CONT_CAPABILITIES_TRIGGERS |
                                                      CONT_CAPABILITIES_DUAL_ANALOG;
pub const CONT_TYPE_POP_N_MUSIC: u32                = CONT_CAPABILITIES_STANDARD_BUTTONS |
                                                      CONT_CAPABILITY_C |
                                                      CONT_CAPABILITIES_DPAD;
pub const CONT_TYPE_DENSHA_DE_GO: u32               = CONT_CAPABILITIES_STANDARD_BUTTONS |
                                                      CONT_CAPABILITIES_EXTENDED_BUTTONS |
                                                      CONT_CAPABILITY_D |
                                                      CONT_CAPABILITIES_DPAD;
pub const CONT_TYPE_PANTHERDC: u32                  = CONT_CAPABILITIES_STANDARD_BUTTONS |
                                                      CONT_CAPABILITIES_EXTENDED_BUTTONS |
                                                      CONT_CAPABILITY_D |
                                                      CONT_CAPABILITIES_TRIGGERS |
                                                      CONT_CAPABILITIES_DPAD |
                                                      CONT_CAPABILITIES_DUAL_ANALOG;


#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn cont_btn_callback(addr: u8, btns: u32, cb: cont_btn_callback_t);
    pub fn cont_has_capabilities(cont: *const maple_device_t, capabilities: u32) -> c_int;
    pub fn cont_is_type(cont: *const maple_device_t, r#type: u32) -> c_int;
    pub fn cont_init();
    pub fn cont_shutdown();
}
