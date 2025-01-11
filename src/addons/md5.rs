// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

#[repr(C)]
pub struct kos_md5_cxt_t {
    pub size:   u64,
    pub hash:   [u32; 4],
    pub buf:    [u8; 64],
}

#[link(name = "kallisti")]
extern "C" {
    pub fn kos_md5_start(cxt: *mut kos_md5_cxt_t);
    pub fn kos_md5_hash_block(cxt: *mut kos_md5_cxt_t, input: *const u8, size: u32);
    pub fn kos_md5_finish(cxt: *mut kos_md5_cxt_t, output: *mut u8);
    pub fn kos_md5(input: *const u8, size: u32, output: *mut u8);
}
