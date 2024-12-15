use crate::prelude::*;

pub const _UTSNAME_LENGTH: usize    = 64;

#[repr(C)]
pub struct utsname {
    sysname:    [c_char; _UTSNAME_LENGTH],
    nodename:   [c_char; _UTSNAME_LENGTH],
    release:    [c_char; _UTSNAME_LENGTH],
    version:    [c_char; _UTSNAME_LENGTH],
    machine:    [c_char; _UTSNAME_LENGTH],
}

extern "C" {
    pub fn uname(n: *mut utsname);
}
