use crate::prelude::*;

pub const NCCS: usize   = 32;

pub type tcflag_t = u32;
pub type cc_t = u8;
pub type speed_t = u32;

#[repr(C)]
pub struct termios {
    pub c_iflag:    tcflag_t,
    pub c_oflag:    tcflag_t,
    pub c_cflag:    tcflag_t,
    pub c_lflag:    tcflag_t,
    pub c_cc:       [cc_t; NCCS],
    pub c_ispeed:   speed_t,
    pub c_ospeed:   speed_t,
}

extern "C" {
    pub fn tcgetattr(fd: c_int, termios_p: *mut termios) -> c_int;
}
