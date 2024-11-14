use crate::prelude::*;

extern "C" {
    pub fn ide_read(linear: u32, numsects: u32, bufptr: *mut c_void) -> c_int;
    pub fn ide_write(linear: u32, numsects: u32, bufptr: *mut c_void) -> c_int;
    pub fn ide_num_sectors() -> u32;
    pub fn ide_init() -> c_int;
    pub fn ide_shutdown();
}
