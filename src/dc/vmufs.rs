// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;
use super::maple::maple_device_t;

#[repr(C, packed)]
pub struct vmu_timestamp_t {
    pub cent:           u8,
    pub year:           u8,
    pub month:          u8,
    pub day:            u8,
    pub hour:           u8,
    pub min:            u8,
    pub sec:            u8,
    pub dow:            u8,
}

#[repr(C, packed)]
pub struct vmu_root_t {
    pub magic:          [u8; 16],
    pub use_custom:     u8,
    pub custom_color:   [u8; 4],
    pub pad1:           [u8; 27],
    pub timestamp:      vmu_timestamp_t,
    pub pad2:           [u8; 8],
    pub unk1:           [u8; 6],
    pub fat_loc:        u16,
    pub fat_size:       u16,
    pub dir_loc:        u16,
    pub dir_size:       u16,
    pub icon_shape:     u16,
    pub blk_cnt:        u16,
    pub unk2:           [u8; 430],
}

#[repr(C, packed)]
pub struct vmu_dir_t {
    pub filetype:       u8,
    pub copyprotect:    u8,
    pub firstblk:       u16,
    pub filename:       [c_char; 12],
    pub timestamp:      vmu_timestamp_t,
    pub filesize:       u16,
    pub hdroff:         u16,
    pub dirty:          u8,
    pub pad1:           [u8; 3],
}

#[link(name = "kallisti")]
extern "C" {
    pub fn vmufs_dir_fill_time(d: *mut vmu_dir_t);
    pub fn vmufs_root_read(dev: *mut maple_device_t, root_buf: *mut vmu_root_t) -> c_int;
    pub fn vmufs_root_write(dev: *mut maple_device_t, root_buf: *mut vmu_root_t) -> c_int;
    pub fn vmu_dir_blocks(root_buf: *mut vmu_root_t) -> c_int;
    pub fn vmu_fat_blocks(root_buf: *mut vmu_root_t) -> c_int;
    pub fn vmu_dir_read(dev: *mut maple_device_t, root_buf: *mut vmu_root_t,
                        dir_buf: *mut vmu_dir_t) -> c_int;
    pub fn vmu_dir_write(dev: *mut maple_device_t, root: *mut vmu_root_t,
                         dir_buf: *mut vmu_dir_t) -> c_int;
    pub fn vmufs_fat_read(dev: *mut maple_device_t, root: *mut vmu_root_t,
                          fat_buf: *mut u16) -> c_int;
    pub fn vmufs_fat_write(dev: *mut maple_device_t, root: *mut vmu_root_t,
                           fat_buf: *mut u16) -> c_int;
    pub fn vmufs_dir_find(root: *mut vmu_root_t, dir: *mut vmu_dir_t,
                          r#fn: *const c_char) -> c_int;
    pub fn vmufs_dir_add(root: *mut vmu_root_t, dir: *mut vmu_dir_t,
                         newdirent: *mut vmu_dir_t) -> c_int;
    pub fn vmufs_file_read(dev: * mut maple_device_t, fat: *mut u16,
                           dirent: *mut vmu_dir_t, outbuf: *mut c_void) -> c_int;
    pub fn vmufs_file_write(dev: *mut maple_device_t, root: *mut vmu_root_t,
                            fat: *mut u16, dir: *mut vmu_dir_t, newdirent: *mut vmu_dir_t,
                            filebuf: *mut c_void, size: c_int) -> c_int;
    pub fn vmufs_file_delete(root: *mut vmu_root_t, fat: *mut u16, dir: *mut vmu_dir_t,
                             r#fn: *const c_char) -> c_int;
    pub fn vmufs_fat_free(root: *mut vmu_root_t, fat: *mut u16) -> c_int;
    pub fn vmufs_dir_free(root: *mut vmu_root_t, dir: *mut vmu_dir_t) -> c_int;
    pub fn vmufs_mutex_lock() -> c_int;
    pub fn vmufs_mutex_unlock() -> c_int;
}
