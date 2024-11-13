use crate::prelude::*;

extern "C" {
    pub fn iso_reset() -> c_int;
    pub fn fs_iso9660_init();
    pub fn fs_iso9660_shutdown();
}