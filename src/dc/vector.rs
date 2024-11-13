#![allow(non_camel_case_types)]

use crate::prelude::*;

#[repr(C, align(8))]
pub struct matrix_t(pub [[f32; 4]; 4]);

#[repr(C)]
pub struct vector_t {
    x:  c_float,
    y:  c_float,
    z:  c_float,
    w:  c_float,
}

pub type point_t = vector_t;