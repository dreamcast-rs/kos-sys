use crate::prelude::*;

extern "C" {
    pub static dbgio_fb: crate::os::dbgio::dbgio_handler_t;
    pub fn dbgio_fb_set_target(t: *mut u16, w: c_int, h: c_int,
                               borderx: c_int, bordery: c_int);
}
