// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[repr(C)]
pub struct vec3f_t {
    pub x:  c_float,
    pub y:  c_float,
    pub z:  c_float,
}

pub const R_DEG: c_float =  182.04444443623349541909523793743;
pub const R_RAD: c_float =  10430.37835;

#[link(name = "kallisti")]
extern "C" {
    #[link_name = "vec_fipr_wrapper"]
    pub fn vec_fipr(vec: vec3f_t);
    #[link_name = "vec_dot_wrapper"]
    pub fn vec_dot(vec1: vec3f_t, vec2: vec3f_t);
    #[link_name = "vec_length_wrapper"]
    pub fn vec_length(vec: vec3f_t);
    #[link_name = "vec_distance_wrapper"]
    pub fn vec_distance(vec1: vec3f_t, vec2: vec3f_t);
    #[link_name = "vec_normalize_wrapper"]
    pub fn vec_normalize(vec: vec3f_t);
    #[link_name = "vec_sub_normalize_wrapper"]
    pub fn vec_sub_normalize(vec1: vec3f_t, vec2: vec3f_t);
    #[link_name = "vec_rotr_xy_wrapper"]
    pub fn vec_rotr_xy(vec: vec3f_t, origin: vec3f_t, angle: c_float);
    #[link_name = "vec_rotr_xz_wrapper"]
    pub fn vec_rotr_xz(vec: vec3f_t, origin: vec3f_t, angle: c_float);
    #[link_name = "vec_rotr_yz_wrapper"]
    pub fn vec_rotr_yz(vec: vec3f_t, origin: vec3f_t, angle: c_float);
    #[link_name = "vec_rotd_xy_wrapper"]
    pub fn vec_rotd_xy(vec: vec3f_t, origin: vec3f_t, angle: c_float);
    #[link_name = "vec_rotd_xz_wrapper"]
    pub fn vec_rotd_xz(vec: vec3f_t, origin: vec3f_t, angle: c_float);
    #[link_name = "vec_rotd_yz_wrapper"]
    pub fn vec_rotd_yz(vec: vec3f_t, origin: vec3f_t, angle: c_float);
    #[link_name = "vec3f_dot_wrapper"]
    pub fn vec3f_dot(x1: c_float, y1: c_float, z1: c_float,
                     x2: c_float, y2: c_float, z2: c_float, w: c_float);
    #[link_name = "vec3f_length_wrapper"]
    pub fn vec3f_length(x: c_float, y: c_float, z: c_float, w: c_float);
    #[link_name = "vec3f_distance_wrapper"]
    pub fn vec3f_distance(x1: c_float, y1: c_float, z1: c_float,
                          x2: c_float, y2: c_float, z2: c_float, w: c_float);
    #[link_name = "vec3f_normalize_wrapper"]
    pub fn vec3f_normalize(x: c_float, y: c_float, z: c_float);
    #[link_name = "vec3f_sub_normalize_wrapper"]
    pub fn vec3f_sub_normalize(x1: c_float, y1: c_float, z1: c_float,
                               x2: c_float, y2: c_float, z2: c_float,
                               x3: c_float, y3: c_float, z3: c_float);
    #[link_name = "vec3f_rotr_xy_wrapper"]
    pub fn vec3f_rotr_xy(px: c_float, py: c_float, pz: c_float,
                         cx: c_float, cy: c_float, cz: c_float, r: c_float);
    #[link_name = "vec3f_rotr_xz_wrapper"]
    pub fn vec3f_rotr_xz(px: c_float, py: c_float, pz: c_float,
                         cx: c_float, cy: c_float, cz: c_float, r: c_float);
    #[link_name = "vec3f_rotr_yz_wrapper"]
    pub fn vec3f_rotr_yz(px: c_float, py: c_float, pz: c_float,
                         cx: c_float, cy: c_float, cz: c_float, r: c_float);
    #[link_name = "vec3f_rotd_xy_wrapper"]
    pub fn vec3f_rotd_xy(px: c_float, py: c_float, pz: c_float,
                         cx: c_float, cy: c_float, cz: c_float, r: c_float);
    #[link_name = "vec3f_rotd_xz_wrapper"]
    pub fn vec3f_rotd_xz(px: c_float, py: c_float, pz: c_float,
                         cx: c_float, cy: c_float, cz: c_float, r: c_float);
    #[link_name = "vec3f_rotd_yz_wrapper"]
    pub fn vec3f_rotd_yz(px: c_float, py: c_float, pz: c_float,
                         cx: c_float, cy: c_float, cz: c_float, r: c_float);
}
