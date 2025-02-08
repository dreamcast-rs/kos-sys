// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn pvr_fog_table_color(a: c_float, r: c_float, g: c_float, b: c_float);
    pub fn pvr_fog_vertex_color(a: c_float, r: c_float, g: c_float, b: c_float);
    pub fn pvr_fog_far_depth(d: c_float);
    pub fn pvr_fog_table_exp2(density: c_float);
    pub fn pvr_fog_table_exp(density: c_float);
    pub fn pvr_fog_table_linear(start: c_float, end: c_float);
    pub fn pvr_fog_table_custom(table: *mut c_float);
}
