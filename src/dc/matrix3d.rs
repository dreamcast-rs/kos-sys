use crate::dc::vector::{point_t, vector_t};
use crate::prelude::*;

extern "C" {
    pub fn mat_rotate_x(r: c_float);
    pub fn mat_rotate_y(r: c_float);
    pub fn mat_rotate_z(r: c_float);
    pub fn mat_rotate(xr: c_float, yr: c_float, zr: c_float);
    pub fn mat_translate(x: c_float, y: c_float, z: c_float);
    pub fn mat_scale(x: c_float, y: c_float, z: c_float);
    pub fn mat_perspective(xcenter: c_float, ycenter: c_float, cot_fovy_2: c_float,
                           znear: c_float, zfar: c_float);
    pub fn mat_lookat(eye: *const point_t, center: *const point_t, up: *const vector_t);
}