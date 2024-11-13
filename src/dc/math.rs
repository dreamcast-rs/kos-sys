use crate::prelude::*;

extern "C" {
    pub fn bit_reverse(value: c_uint) -> c_uint;
}