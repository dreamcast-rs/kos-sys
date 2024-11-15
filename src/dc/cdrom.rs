use crate::prelude::*;

pub const CMD_CHECK_LICENSE: c_int              = 2;
pub const CMD_REQ_SPI_CMD: c_int                = 4;
pub const CMD_PIOREAD: c_int                    = 16;
pub const CMD_DMAREAD: c_int                    = 17;
pub const CMD_GETTOC: c_int                     = 18;
pub const CMD_GETTOC2: c_int                    = 19;
pub const CMD_PLAY: c_int                       = 20;
pub const CMD_PLAY2: c_int                      = 21;
pub const CMD_PAUSE: c_int                      = 22;
pub const CMD_RELEASE: c_int                    = 23;
pub const CMD_INIT: c_int                       = 24;
pub const CMD_DMA_ABORT: c_int                  = 25;
pub const CMD_OPEN_TRAY: c_int                  = 26;
pub const CMD_SEEK: c_int                       = 27;
pub const CMD_DMAREAD_STREAM: c_int             = 28;
pub const CMD_NOP: c_int                        = 29;
pub const CMD_REQ_MODE: c_int                   = 30;
pub const CMD_SET_MODE: c_int                   = 31;
pub const CMD_SCAN_CD: c_int                    = 32;
pub const CMD_STOP: c_int                       = 33;
pub const CMD_GETSCD: c_int                     = 34;
pub const CMD_GETSES: c_int                     = 35;
pub const CMD_REQ_STAT: c_int                   = 36;
pub const CMD_PIOREAD_STREAM: c_int             = 37;
pub const CMD_DMAREAD_STREAM_EX: c_int          = 38;
pub const CMD_PIOREAD_STREAM_EX: c_int          = 39;
pub const CMD_GET_VERS: c_int                   = 40;
pub const CMD_MAX: c_int                        = 47;

pub const ERR_OK: c_int                         = 0;
pub const ERR_NO_DISC: c_int                    = 1;
pub const ERR_DISC_CHG: c_int                   = 2;
pub const ERR_SYS: c_int                        = 3;
pub const ERR_ABORTED: c_int                    = 4;
pub const ERR_NO_ACTIVE: c_int                  = 5;
pub const ERR_TIMEOUT: c_int                    = 6;

pub const FAILED: c_int                         = -1;
pub const NO_ACTIVE: c_int                      = 0;
pub const PROCESSING: c_int                     = 1;
pub const COMPLETED: c_int                      = 2;
pub const STREAMING: c_int                      = 3;
pub const BUSY: c_int                           = 4;

pub const ATA_STAT_INTERNAL: c_int              = 0x00;
pub const ATA_STAT_IRQ: c_int                   = 0x01;
pub const ATA_STAT_DRQ_0: c_int                 = 0x02;
pub const ATA_STAT_DRQ_1: c_int                 = 0x03;
pub const ATA_STAT_BUSY: c_int                  = 0x04;

pub const CDDA_TRACKS: c_int                    = 1;
pub const CDDA_SECTORS: c_int                   = 2;

pub const CDROM_READ_WHOLE_SECTOR: c_int        = 0x1000;
pub const CDROM_READ_DATA_AREA: c_int           = 0x2000;

pub const CD_SUB_Q_ALL: c_int                   = 0;
pub const CD_SUB_Q_CHANNEL: c_int               = 1;
pub const CD_SUB_MEDIA_CATALOG: c_int           = 2;
pub const CD_SUB_TRACK_ISRC: c_int              = 3;
pub const CD_SUB_RESERVED: c_int                = 4;

pub const CD_SUB_AUDIO_STATUS_INVALID: c_int    = 0x00;
pub const CD_SUB_AUDIO_STATUS_PLAYING: c_int    = 0x11;
pub const CD_SUB_AUDIO_STATUS_PAUSED: c_int     = 0x12;
pub const CD_SUB_AUDIO_STATUS_ENDED: c_int      = 0x13;
pub const CD_SUB_AUDIO_STATUS_ERROR: c_int      = 0x14;
pub const CD_SUB_AUDIO_STATUS_NO_INFO: c_int    = 0x15;

pub const CDROM_READ_PIO: c_int                 = 0;
pub const CDROM_READ_DMA: c_int                 = 1;

pub const CD_STATUS_READ_FAIL: c_int            = -1;
pub const CD_STATUS_BUSY: c_int                 = 0;
pub const CD_STATUS_PAUSED: c_int               = 1;
pub const CD_STATUS_STANDBY: c_int              = 2;
pub const CD_STATUS_PLAYING: c_int              = 3;
pub const CD_STATUS_SEEKING: c_int              = 4;
pub const CD_STATUS_SCANNING: c_int             = 5;
pub const CD_STATUS_OPEN: c_int                 = 6;
pub const CD_STATUS_NO_DISC: c_int              = 7;
pub const CD_STATUS_RETRY: c_int                = 8;
pub const CD_STATUS_ERROR: c_int                = 9;
pub const CD_STATUS_FATAL: c_int                = 12;

pub const CD_CDDA: c_int                        = 0x00;
pub const CD_CDROM: c_int                       = 0x10;
pub const CD_CDROM_XA: c_int                    = 0x20;
pub const CD_CDI: c_int                         = 0x30;
pub const CD_GDROM: c_int                       = 0x80;
pub const CD_FAIL: c_int                        = 0xF0;

#[repr(C)]
pub struct CDROM_TOC {
    pub entry:          [u32; 99],
    pub first:          u32,
    pub last:           u32,
    pub leadout_sector: u32,
}

#[macro_export]
macro_rules! TOC_LBA {
    ($n:expr) => {
        ($n & 0x00FF_FFFF)
    };
}

#[macro_export]
macro_rules! TOC_ADR {
    ($n:expr) => {
        (($n & 0x0F00_0000) >> 24)
    };
}

#[macro_export]
macro_rules! TOC_CTRL {
    ($n:expr) => {
        (($n & 0xF000_0000) >> 28)
    };
}

#[macro_export]
macro_rules! TOC_TRACK {
    ($n:expr) => {
        (($n & 0x00FF_0000) >> 16)
    };
}

extern "C" {
    pub fn cdrom_set_sector_size(size: c_int) -> c_int;
    pub fn cdrom_exec_cmd(cmd: c_int, param: *mut c_void) -> c_int;
    pub fn cdrom_exec_cmd_timed(cmd: c_int, param: *mut c_void, timeout: c_int) -> c_int;
    pub fn cdrom_get_status(status: *mut c_int, disc_type: *mut c_int) -> c_int;
    pub fn cdrom_change_dataype(sector_part: c_int, cdxa: c_int,
                                sector_size: c_int) -> c_int;
    pub fn cdrom_change_datatype(sector_part: c_int, cdxa: c_int,
                                 sector_size: c_int) -> c_int;
    pub fn cdrom_reinit() -> c_int;
    pub fn cdrom_reinit_ex(sector_part: c_int, cdxa: c_int, sector_size: c_int) -> c_int;
    pub fn cdrom_read_toc(toc_buffer: *mut CDROM_TOC, session: c_int) -> c_int;
    pub fn cdrom_read_sectors_ex(buffer: *mut c_void, sector: c_int,
                                 cnt: c_int, mode: c_int) -> c_int;
    pub fn cdrom_read_sectors(buffer: *mut c_void, sector: c_int, cnt: c_int) -> c_int;
    pub fn cdrom_get_subcode(buffer: *mut c_void, buflen: c_int, which: c_int) -> c_int;
    pub fn cdrom_locate_data_track(toc: *mut CDROM_TOC) -> u32;
    pub fn cdrom_cdda_play(start: u32, end: u32, loops: u32, mode: c_int) -> c_int;
    pub fn cdrom_cdda_pause() -> c_int;
    pub fn cdrom_cdda_resume() -> c_int;
    pub fn cdrom_spin_down() -> c_int;
    pub fn cdrom_init();
    pub fn cdrom_shutdown();
}