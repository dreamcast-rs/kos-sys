// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::dc::vector::point_t;
use crate::prelude::*;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn bspline_coeff(pnt: *const point_t);
    pub fn bspline_get_point(t: c_float, p: *mut point_t);
}
