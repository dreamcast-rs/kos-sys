use crate::prelude::*;

extern "C" {
    pub fn vblank_handler_add(hnd: super::asic::asic_evt_handler,
                              data: *mut c_void) -> c_int;
    pub fn vblank_handler_remove(handle: c_int) -> c_int;
    pub fn vblank_init() -> c_int;
    pub fn vblank_shutdown() -> c_int;
}