use crate::prelude::*;

#[repr(C)]
pub struct vec3f_t {
    x:  c_float,
    y:  c_float,
    z:  c_float,
}

pub const R_DEG: c_float =  182.04444443623349541909523793743;
pub const R_RAD: c_float =  10430.37835;

extern "C" {
    #[link_name = "vec3f_dot_stub"]
    pub fn vec3f_dot(x1: c_float, y1: c_float, z1: c_float,
                     x2: c_float, y2: c_float, z2: c_float, w: c_float);
    #[link_name = "vec3f_length_stub"]
    pub fn vec3f_length(x: c_float, y: c_float, z: c_float, w: c_float);
    #[link_name = "vec3f_distance_stub"]
    pub fn vec3f_distance(x1: c_float, y1: c_float, z1: c_float,
                          x2: c_float, y2: c_float, z2: c_float, w: c_float);
    #[link_name = "vec3f_normalize_stub"]
    pub fn vec3f_normalize(x: c_float, y: c_float, z: c_float);
    #[link_name = "vec3f_sub_normalize_stub"]
    pub fn vec3f_sub_normalize(x1: c_float, y1: c_float, z1: c_float,
                               x2: c_float, y2: c_float, z2: c_float,
                               x3: c_float, y3: c_float, z3: c_float);
    #[link_name = "vec3f_rotr_xy_stub"]
    pub fn vec3f_rotr_xy(px: c_float, py: c_float, pz: c_float,
                         cx: c_float, cy: c_float, cz: c_float, r: c_float);
    #[link_name = "vec3f_rotr_xz_stub"]
    pub fn vec3f_rotr_xz(px: c_float, py: c_float, pz: c_float,
                         cx: c_float, cy: c_float, cz: c_float, r: c_float);
    #[link_name = "vec3f_rotr_yz_stub"]
    pub fn vec3f_rotr_yz(px: c_float, py: c_float, pz: c_float,
                         cx: c_float, cy: c_float, cz: c_float, r: c_float);
    #[link_name = "vec3f_rotd_xy_stub"]
    pub fn vec3f_rotd_xy(px: c_float, py: c_float, pz: c_float,
                         cx: c_float, cy: c_float, cz: c_float, r: c_float);
    #[link_name = "vec3f_rotd_xz_stub"]
    pub fn vec3f_rotd_xz(px: c_float, py: c_float, pz: c_float,
                         cx: c_float, cy: c_float, cz: c_float, r: c_float);
    #[link_name = "vec3f_rotd_yz_stub"]
    pub fn vec3f_rotd_yz(px: c_float, py: c_float, pz: c_float,
                         cx: c_float, cy: c_float, cz: c_float, r: c_float);
}
