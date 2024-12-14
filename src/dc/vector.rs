use crate::prelude::*;

#[repr(C, align(8))]
pub struct matrix_t(pub [[f32; 4]; 4]);

#[repr(C)]
pub struct vector_t {
    pub x:  c_float,
    pub y:  c_float,
    pub z:  c_float,
    pub w:  c_float,
}

pub type point_t = vector_t;