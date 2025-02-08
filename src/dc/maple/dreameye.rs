// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

use super::maple_device_t;

#[repr(C)]
pub struct dreameye_state_t {
    pub image_count:        c_int,
    pub image_count_valid:  c_int,
    pub transfer_count:     c_int,
    pub img_transferring:   c_int,
    pub img_buf:            *mut u8,
    pub img_size:           c_int,
    pub img_number:         u8,
}

pub const DREAMEYE_GETCOND_NUM_IMAGES: c_int        = 0x81;
pub const DREAMEYE_GETCOND_TRANSFER_COUNT: c_int    = 0x83;
pub const DREAMEYE_SUBCOMMAND_IMAGEREQ: c_int       = 0x04;
pub const DREAMEYE_SUBCOMMAND_ERASE: c_int          = 0x05;
pub const DREAMEYE_SUBCOMMAND_ERROR: c_int          = 0xFF;
pub const DREAMEYE_IMAGEREQ_CONTINUE: c_int         = 0x00;
pub const DREAMEYE_IMAGEREQ_START: c_int            = 0x40;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn dreameye_get_image_count(dev: *mut maple_device_t, block: c_int) -> c_int;
    pub fn dreameye_get_image(
        dev: *mut maple_device_t,
        image: u8,
        data: *mut *mut u8,
        img_sz: *mut c_int,
    ) -> c_int;
    pub fn dreameye_erase_image(dev: *mut maple_device_t, image: u8, block: c_int) -> c_int;
    pub fn dreameye_init();
    pub fn dreameye_shutdown();
}
