use crate::dc::vector::point_t;
use crate::prelude::*;

extern "C" {
    pub fn bspline_coeff(pnt: *const point_t);
    pub fn bspline_get_point(t: c_float, p: *mut point_t);
}
