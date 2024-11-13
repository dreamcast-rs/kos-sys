use crate::prelude::*;

pub const F_PI: c_float = 3.1415926;

extern "C" {
    #[link_name = "fipr_stub"]
    pub fn fipr(x: c_float, y: c_float, z: c_float, w: c_float,
                a: c_float, b: c_float, c: c_float, d: c_float) -> c_float;
    #[link_name = "fipr_magnitude_sqr_stub"]
    pub fn fiper_magnitude_sqr(x: c_float, y: c_float, z: c_float, w: c_float) -> c_float;
    #[link_name = "fsin_stub"]
    pub fn fsin(r: c_float) -> c_float;
    #[link_name = "fcos_stub"]
    pub fn fcos(r: c_float) -> c_float;
    #[link_name = "ftan_stub"]
    pub fn ftan(r: c_float) -> c_float;
    #[link_name = "fisin_stub"]
    pub fn fisin(d: c_int) -> c_float;
    #[link_name = "ficos_stub"]
    pub fn ficos(d: c_int) -> c_float;
    #[link_name = "fitan_stub"]
    pub fn fitan(d: c_int) -> c_float;
    #[link_name = "fsqrt_stub"]
    pub fn fsqrt(f: c_float) -> c_float;
    #[link_name = "frsqrt_stub"]
    pub fn frsqrt(f: c_float) -> c_float;
    #[link_name = "fsincos_stub"]
    pub fn fsincos(f: c_float, s: *mut c_float, c: *mut c_float);
    #[link_name = "fsincosr_stub"]
    pub fn fsincosr(f: c_float, s: *mut c_float, c: *mut c_float);
    #[link_name = "pvr_pack_bump_stub"]
    pub fn pvr_pack_bump(h: c_float, t: c_float, q: c_float) -> u32;
}