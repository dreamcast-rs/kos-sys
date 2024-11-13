use crate::prelude::*;

extern "C" {
    pub fn fs_vmu_init() -> c_int;
    pub fn fs_vmu_shutdown() -> c_int;
}