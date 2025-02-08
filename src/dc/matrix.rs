// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

use crate::dc::vector::{matrix_t, vector_t};

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn mat_store(out: *mut matrix_t);
    pub fn mat_load(src: *const matrix_t);
    pub fn mat_identity();
    pub fn mat_apply(src: *const matrix_t);
    pub fn mat_transform(
        invecs: *const vector_t,
        outvecs: *mut vector_t,
        veccnt: c_int,
        stride: c_int,
    );
    pub fn mat_transform_sq(input: *mut c_void, output: *mut c_void, veccnt: c_int);
    #[link_name = "mat_trans_single_wrapper"]
    pub fn mat_trans_single(x: c_float, y: c_float, z: c_float);
    #[link_name = "mat_trans_single4_wrapper"]
    pub fn mat_trans_single4(x: c_float, y: c_float, z: c_float, w: c_float);
    #[link_name = "mat_trans_single3_wrapper"]
    pub fn mat_trans_single3(x: c_float, y: c_float, z: c_float);
    #[link_name = "mat_trans_nodiv_wrapper"]
    pub fn mat_trans_nodiv(x: c_float, y: c_float, z: c_float, w: c_float);
    #[link_name = "mat_trans_single3_nodiv_wrapper"]
    pub fn mat_trans_single3_nodiv(x: c_float, y: c_float, z: c_float);
    #[link_name = "mat_trans_single3_nomod_wrapper"]
    pub fn mat_trans_single3_nomod(
        x: c_float,
        y: c_float,
        z: c_float,
        x2: c_float,
        y2: c_float,
        z2: c_float,
    );
    #[link_name = "mat_trans_single3_nodiv_nomod_wrapper"]
    pub fn mat_trans_single3_nodiv_nomod(
        x: c_float,
        y: c_float,
        z: c_float,
        x2: c_float,
        y2: c_float,
        z2: c_float,
    );
    #[link_name = "mat_trans_single3_nodivw_wrapper"]
    pub fn mat_trans_single3_nodivw(x: c_float, y: c_float, z: c_float, w: c_float);
    #[link_name = "mat_trans_single3_nodiv_div_wrapper"]
    pub fn mat_trans_single3_nodiv_div(
        x: c_float,
        y: c_float,
        z: c_float,
        xd: c_float,
        yd: c_float,
        zd: c_float,
    );
    #[link_name = "mat_trans_normal3_wrapper"]
    pub fn mat_trans_normal3(x: c_float, y: c_float, z: c_float);
    #[link_name = "mat_trans_normal3_nomod_wrapper"]
    pub fn mat_trans_normal3_nomod(
        x: c_float,
        y: c_float,
        z: c_float,
        x2: c_float,
        y2: c_float,
        z2: c_float,
    );
}
