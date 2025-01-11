// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

use crate::os::blockdev::kos_blockdev_t;
use crate::prelude::*;

pub const G1_ATA_MASTER: u8     = 0x00;
pub const G1_ATA_MASTER_ALT: u8 = 0x90;
pub const G1_ATA_SLAVE: u8      = 0xB0;
pub const G1_ATA_LBA_MODE: u8   = 0x40;

#[link(name = "kallisti")]
extern "C" {
    pub fn g1_dma_in_progress() -> c_int;
    pub fn g1_ata_mutex_lock() -> c_int;
    pub fn g1_ata_mutex_unlock() -> c_int;
    pub fn g1_ata_select_device(dev: u8) -> u8;
    pub fn g1_ata_read_chs(c: u16, h: u8, s: u8, count: c_size_t,
                           buf: *mut c_void) -> c_int;
    pub fn g1_ata_write_chs(c: u16, h: u8, s: u8, count: c_size_t,
                            buf: *const c_void) -> c_int;
    pub fn g1_ata_read_lba(sector: u64, count: c_size_t,
                           buf: *mut c_void) -> c_int;
    pub fn g1_ata_read_lba_dma(sector: u64, count: c_size_t,
                               buf: *mut c_void, block: c_int) -> c_int;
    pub fn g1_ata_write_lba(sector: u64, count: c_size_t,
                            buf: *const c_void) -> c_int;
    pub fn g1_ata_write_lba_dma(sector: u64, count: c_size_t,
                                buf: *const c_void, block: c_int) -> c_int;
    pub fn g1_ata_flush() -> c_int;
    pub fn g1_ata_lba_mode() -> c_int;
    pub fn g1_ata_blockdev_for_partition(partition: c_int, dma: c_int,
                                         rv: *mut kos_blockdev_t,
                                         partition_type: *mut u8) -> c_int;
    pub fn g1_ata_init() -> c_int;
    pub fn g1_ata_shutdown();
}
