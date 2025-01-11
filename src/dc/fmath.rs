// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub const F_PI: c_float = 3.1415926;

#[link(name = "kallisti")]
extern "C" {
    #[link_name = "fipr_wrapper"]
    pub fn fipr(x: c_float, y: c_float, z: c_float, w: c_float,
                a: c_float, b: c_float, c: c_float, d: c_float) -> c_float;
    #[link_name = "fipr_magnitude_sqr_wrapper"]
    pub fn fiper_magnitude_sqr(x: c_float, y: c_float, z: c_float, w: c_float) -> c_float;
    #[link_name = "fsin_wrapper"]
    pub fn fsin(r: c_float) -> c_float;
    #[link_name = "fcos_wrapper"]
    pub fn fcos(r: c_float) -> c_float;
    #[link_name = "ftan_wrapper"]
    pub fn ftan(r: c_float) -> c_float;
    #[link_name = "fisin_wrapper"]
    pub fn fisin(d: c_int) -> c_float;
    #[link_name = "ficos_wrapper"]
    pub fn ficos(d: c_int) -> c_float;
    #[link_name = "fitan_wrapper"]
    pub fn fitan(d: c_int) -> c_float;
    #[link_name = "fsqrt_wrapper"]
    pub fn fsqrt(f: c_float) -> c_float;
    #[link_name = "frsqrt_wrapper"]
    pub fn frsqrt(f: c_float) -> c_float;
    #[link_name = "fsincos_wrapper"]
    pub fn fsincos(f: c_float, s: *mut c_float, c: *mut c_float);
    #[link_name = "fsincosr_wrapper"]
    pub fn fsincosr(f: c_float, s: *mut c_float, c: *mut c_float);
    #[link_name = "pvr_pack_bump_wrapper"]
    pub fn pvr_pack_bump(h: c_float, t: c_float, q: c_float) -> u32;
}
