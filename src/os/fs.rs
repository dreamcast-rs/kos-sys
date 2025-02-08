// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[repr(C)]
pub struct dirent_t {
    pub size:       c_int,
    pub name:       [c_char; super::limits::NAME_MAX],
    pub time:       time_t,
    pub attr:       u32,
}

pub const STAT_UNIQUE_NONE: u32     = 0;

pub const STAT_TYPE_NONE: u32       = 0;
pub const STAT_TYPE_FILE: u32       = 1;
pub const STAT_TYPE_DIR: u32        = 2;
pub const STAT_TYPE_PIPE: u32       = 3;
pub const STAT_TYPE_META: u32       = 4;
pub const STAT_TYPE_SYMLINK: u32    = 5;

pub const STAT_ATTR_NONE: u32       = 0x00;
pub const STAT_ATTR_R: u32          = 0x01;
pub const STAT_ATTR_W: u32          = 0x02;

pub const STAT_ATTR_RW: u32         = STAT_ATTR_R | STAT_ATTR_W;

pub type file_t = c_int;

pub const FILEHND_INVALID: file_t   = -1;

#[repr(C)]
pub struct vfs_handler_t {
    pub nmmgr:      crate::os::nmmgr::nmmgr_handler_t,
    pub cache:      c_int,
    pub privdata:   *mut c_void,
    pub open:       Option<unsafe extern "C" fn(vfs: vfs_handler_t, r#fn: *const c_char,
                                                mode: c_int) -> *mut c_void>,
    pub close:      Option<unsafe extern "C" fn(hnd: *mut c_void) -> c_int>,
    pub read:       Option<unsafe extern "C" fn(hnd: *mut c_void, buffer: *mut c_void,
                                                cnt: c_size_t) -> c_ssize_t>,
    pub write:      Option<unsafe extern "C" fn(hnd: *mut c_void, buffer: *const c_void,
                                                cnt: c_size_t) -> c_ssize_t>,
    pub seek:       Option<unsafe extern "C" fn(hnd: *mut c_void, offset: off_t,
                                                whence: c_int) -> off_t>,
    pub tell:       Option<unsafe extern "C" fn(hnd: *mut c_void) -> off_t>,
    pub total:      Option<unsafe extern "C" fn(hnd: *mut c_void) -> c_size_t>,
    pub readdir:    Option<unsafe extern "C" fn(hnd: *mut c_void) -> *mut dirent_t>,
    pub ioctl:      Option<unsafe extern "C" fn(hnd: *mut c_void, cmd: c_int,
                                                ap: VaList) -> c_int>,
    pub rename:     Option<unsafe extern "C" fn(vfs: *mut vfs_handler_t,
                                                fn1: *const c_char, fn2: *const c_char)
                                                -> c_int>,
    pub unlink:     Option<unsafe extern "C" fn(vfs: *mut vfs_handler_t,
                                                r#fn: *const c_char) -> c_int>,
    pub mmap:       Option<unsafe extern "C" fn(fd: *mut c_void) -> *mut c_void>,
    pub complete:   Option<unsafe extern "C" fn(fd: *mut c_void,
                                                rv: *mut c_ssize_t) -> c_int>,
    pub stat:       Option<unsafe extern "C" fn(vfs: *mut vfs_handler_t,
                                                path: *const c_char, buf: *mut stat,
                                                flag: c_int) -> c_int>,
    pub mkdir:      Option<unsafe extern "C" fn(vfs: *mut vfs_handler_t,
                                                r#fn: *const c_char) -> c_int>,
    pub rmdir:      Option<unsafe extern "C" fn(vfs: *mut vfs_handler_t,
                                                r#fn: *const c_char) -> c_int>,
    pub fcntl:      Option<unsafe extern "C" fn(fd: *mut c_void, cmd: c_int,
                                                ap: VaList) -> c_int>,
    pub poll:       Option<unsafe extern "C" fn(fd: *mut c_void,
                                                events: c_short) -> c_short>,
    pub link:       Option<unsafe extern "C" fn(vfs: *mut vfs_handler_t,
                                                path1: *const c_char,
                                                path2: *const c_char) -> c_int>,
    pub symlink:    Option<unsafe extern "C" fn(vfs: *mut vfs_handler_t,
                                                path1: *const c_char,
                                                path2: *const c_char) -> c_int>,
    pub seek64:     Option<unsafe extern "C" fn(hnd: *mut c_void, offset: off64_t,
                                                whence: c_int) -> off64_t>,
    pub tell64:     Option<unsafe extern "C" fn(hnd: *mut c_void) -> off64_t>,
    pub total64:    Option<unsafe extern "C" fn(hnd: *mut c_void) -> u64>,
    pub readlink:   Option<unsafe extern "C" fn(vfs: *mut vfs_handler_t,
                                                path: *const c_char, buf: *mut c_char,
                                                bufsize: c_size_t) -> c_ssize_t>,
    pub rewinddir:  Option<unsafe extern "C" fn(hnd: *mut c_void) -> c_int>,
    pub fstat:      Option<unsafe extern "C" fn(hnd: *mut c_void,
                                                st: *mut stat) -> c_int>,
}

pub const O_MODE_MASK: c_int        = 0x0f;
pub const O_ASYNC: c_int            = 0x0200;
pub const O_DIR: c_int              = 0x1000;
pub const O_META: c_int             = 0x2000;

pub const SEEK_SET: c_int           = 0;
pub const SEEK_CUR: c_int           = 1;
pub const SEEK_END: c_int           = 2;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub static mut fd_table: [*mut c_void; super::opts::FD_SETSIZE];
    pub fn fs_open(r#fn: *const c_char, mode: c_int) -> file_t;
    pub fn fs_close(hnd: file_t) -> c_int;
    pub fn fs_read(hnd: file_t, buffer: *mut c_void, cnt: c_size_t) -> c_ssize_t;
    pub fn fs_write(hnd: file_t, buffer: *const c_void, cnt: c_size_t) -> c_ssize_t;
    pub fn fs_seek(hnd: file_t, offset: off_t, whence: c_int) -> off_t;
    pub fn fs_seek64(hnd: file_t, offset: off64_t, whence: c_int) -> off64_t;
    pub fn fs_tell(hnd: file_t) -> off_t;
    pub fn fs_tell64(hnd: file_t) -> off64_t;
    pub fn fs_total(hnd: file_t) -> c_size_t;
    pub fn fs_total64(hnd: file_t) -> u64;
    pub fn fs_readdir(hnd: file_t) -> *mut dirent_t;
    pub fn fs_ioctl(hnd: file_t, cmd: c_int, ...) -> c_int;
    pub fn fs_rename(fn1: *const c_char, fn2: *const c_char) -> c_int;
    pub fn fs_unlink(r#fn: *const c_char) -> c_int;
    pub fn fs_chdir(r#fn: *const c_char) -> c_int;
    pub fn fs_mmap(hnd: file_t) -> *mut c_void;
    pub fn fs_complete(fd: file_t, rv: *mut c_ssize_t) -> c_int;
    pub fn fs_mkdir(r#fn: *const c_char) -> c_int;
    pub fn fs_rmdir(r#fn: *const c_char) -> c_int;
    pub fn fs_fcntl(fd: file_t, cmd: c_int, ...) -> c_int;
    pub fn fs_link(path1: *const c_char, path2: *const c_char) -> c_int;
    pub fn fs_symlink(path1: *const c_char, path2: *const c_char) -> c_int;
    pub fn fs_readlink(path: *const c_char, buf: *mut c_char, bufsize: c_size_t) -> c_ssize_t;
    pub fn fs_stat(path: *const c_char, buf: *mut stat, flag: c_int) -> c_int;
    pub fn fs_rewinddir(hnd: file_t) -> c_int;
    pub fn fs_fstat(hnd: file_t, buf: *mut stat) -> c_int;
    pub fn fs_dup(oldfd: file_t) -> file_t;
    pub fn fs_dup2(oldfd: file_t, newfd: file_t) -> file_t;
    pub fn fs_open_handle(vfs: *mut vfs_handler_t, hnd: *mut c_void) -> file_t;
    pub fn fs_get_handler(fd: file_t) -> *mut vfs_handler_t;
    pub fn fs_get_handle(fd: file_t) -> *mut c_void;
    pub fn fs_getwd() -> *const c_char;
    pub fn fs_copy(src: *const c_char, dst: *const c_char) -> c_ssize_t;
    pub fn fs_load(src: *const c_char, out_ptr: *mut *mut c_void) -> c_ssize_t;
    pub fn fs_path_append(dst: *mut c_char, src: *const c_char, len: c_size_t) -> c_ssize_t;
    pub fn fs_normalize_path(path: *const c_char, resolved: *mut c_char) -> *mut c_char;
    pub fn fs_init() -> c_int;
    pub fn fs_shutdown();
}
