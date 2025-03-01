// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

use crate::os::blockdev::kos_blockdev_t;

pub const G1_ATA_MASTER: u8                             = 0x00;
pub const G1_ATA_MASTER_ALT: u8                         = 0x90;
pub const G1_ATA_SLAVE: u8                              = 0xB0;
pub const G1_ATA_LBA_MODE: u8                           = 0x40;
pub const G1_ATA_BUS_PROTECTION: u32                    = 0x005F74E4;
pub const G1_ATA_BUS_PROTECTION_STATUS: u32             = 0x005F74EC;
pub const G1_ATA_BUS_PROTECTION_STATUS_IN_PROGRESS: u32 = 0x00;
pub const G1_ATA_BUS_PROTECTION_STATUS_FAILED: u32      = 0x02;
pub const G1_ATA_BUS_PROTECTION_STATUS_PASSED: u32      = 0x03;
pub const G1_ATA_DMA_PROTECTION: u32                    = 0x005F74B8;
pub const G1_ATA_DMA_UNLOCK_CODE: u32                   = 0x8843;
pub const G1_ATA_DMA_UNLOCK_SYSMEM: u32                 = G1_ATA_DMA_UNLOCK_CODE << 16 | 0x407F;
pub const G1_ATA_DMA_UNLOCK_ALLMEM: u32                 = G1_ATA_DMA_UNLOCK_CODE << 16 | 0x007F;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn g1_dma_in_progress() -> c_int;
    pub fn g1_ata_mutex_lock() -> c_int;
    pub fn g1_ata_mutex_unlock() -> c_int;
    pub fn g1_ata_select_device(dev: u8) -> u8;
    pub fn g1_ata_read_chs(c: u16, h: u8, s: u8, count: c_size_t, buf: *mut c_void) -> c_int;
    pub fn g1_ata_write_chs(c: u16, h: u8, s: u8, count: c_size_t, buf: *const c_void) -> c_int;
    pub fn g1_ata_read_lba(sector: u64, count: c_size_t, buf: *mut c_void) -> c_int;
    pub fn g1_ata_read_lba_dma(
        sector: u64,
        count: c_size_t,
        buf: *mut c_void,
        block: c_int,
    ) -> c_int;
    pub fn g1_ata_write_lba(sector: u64, count: c_size_t, buf: *const c_void) -> c_int;
    pub fn g1_ata_write_lba_dma(
        sector: u64,
        count: c_size_t,
        buf: *const c_void,
        block: c_int,
    ) -> c_int;
    pub fn g1_ata_flush() -> c_int;
    pub fn g1_ata_lba_mode() -> c_int;
    pub fn g1_ata_blockdev_for_partition(
        partition: c_int,
        dma: c_int,
        rv: *mut kos_blockdev_t,
        partition_type: *mut u8,
    ) -> c_int;
    pub fn g1_ata_init() -> c_int;
    pub fn g1_ata_shutdown();
}
