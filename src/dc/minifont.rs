use crate::prelude::*;

extern "C" {
    pub fn minifont_draw(buffer: *mut u16, bufwidth: u32, c: u32) -> c_int;
    pub fn minifont_draw_str(b: *mut u16, bufwidth: u32, str: *const c_char) -> c_int;
}