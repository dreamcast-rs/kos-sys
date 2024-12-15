use crate::prelude::*;

extern "C" {
    pub fn arch_exec_at(image: *const c_void, length: u32, address: u32) -> !;
    pub fn arch_exec(image: *const c_void, length: u32) -> !;
}
