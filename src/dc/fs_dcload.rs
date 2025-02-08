// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub const DCLOADMAGICVALUE: u32         = 0xdeadbeef;

pub const DCLOADMAGICADDR: *mut c_uint  = 0x8c004004 as *mut u32;

pub const DCLOAD_TYPE_NONE: c_int       = -1;
pub const DCLOAD_TYPE_SER: c_int        = 0;
pub const DCLOAD_TYPE_IP: c_int         = 1;

pub const DCLOAD_READ: c_uint           = 0;
pub const DCLOAD_WRITE: c_uint          = 1;
pub const DCLOAD_OPEN: c_uint           = 2;
pub const DCLOAD_CLOSE: c_uint          = 3;
pub const DCLOAD_CREAT: c_uint          = 4;
pub const DCLOAD_LINK: c_uint           = 5;
pub const DCLOAD_UNLINK: c_uint         = 6;
pub const DCLOAD_CHDIR: c_uint          = 7;
pub const DCLOAD_CHMOD: c_uint          = 8;
pub const DCLOAD_LSEEK: c_uint          = 9;
pub const DCLOAD_FSTAT: c_uint          = 10;
pub const DCLOAD_TIME: c_uint           = 11;
pub const DCLOAD_STAT: c_uint           = 12;
pub const DCLOAD_UTIME: c_uint          = 13;
pub const DCLOAD_ASSIGNWRKMEM: c_uint   = 14;
pub const DCLOAD_EXIT: c_uint           = 15;
pub const DCLOAD_OPENDIR: c_uint        = 16;
pub const DCLOAD_CLOSEDIR: c_uint       = 17;
pub const DCLOAD_READDIR: c_uint        = 18;
pub const DCLOAD_GETHOSTINFO: c_uint    = 19;
pub const DCLOAD_GDBPACKET: c_uint      = 20;

#[repr(C)]
pub struct dcload_dirent {
    pub d_ino:      c_long,
    pub d_off:      off_t,
    pub d_reclen:   c_ushort,
    pub d_type:     c_uchar,
    pub d_name:     [c_char; 256],
}

pub type dcload_dirent_t = dcload_dirent;

#[repr(C)]
pub struct dcload_stat {
    pub st_dev:     c_ushort,
    pub st_ino:     c_ushort,
    pub st_mode:    c_int,
    pub st_nlink:   c_ushort,
    pub st_uid:     c_ushort,
    pub st_gid:     c_ushort,
    pub st_rdev:    c_ushort,
    pub st_size:    c_long,
    pub atime:      c_long,
    pub st_spare1:  c_long,
    pub mtime:      c_long,
    pub st_spare2:  c_long,
    pub ctime:      c_long,
    pub st_spare3:  c_long,
    pub st_blksize: c_long,
    pub st_blocks:  c_long,
    pub st_spare4:  [c_long; 2],
}

pub type dcload_stat_t = dcload_stat;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub static dbgio_dcload: crate::os::dbgio::dbgio_handler_t;
    pub static dcload_type: c_int;
    pub fn dcloadsyscall(syscall: c_uint, ...) -> c_int;
    pub fn dcload_printk(str: *const c_char);
    pub fn dcload_gdbpacket(
        in_buf: *const c_char,
        in_size: c_size_t,
        out_buf: *mut c_char,
        out_size: c_size_t,
    ) -> c_size_t;
    pub fn fs_dcload_init_console();
    pub fn fs_dcload_init();
    pub fn fs_dcload_shutdown();
}
