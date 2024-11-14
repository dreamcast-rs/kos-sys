use crate::prelude::*;

pub const NETCFG_METHOD_DHCP: c_int     = 0;
pub const NETCFG_METHOD_STATIC: c_int   = 1;
pub const NETCFG_METHOD_PPPOE: c_int    = 4;

pub const NETCFG_SRC_VMU: c_int         = 0;
pub const NETCFG_SRC_FLASH: c_int       = 1;
pub const NETCFG_SRC_CWD: c_int         = 2;
pub const NETCFG_SRC_CDROOT: c_int      = 3;

#[repr(C)]
pub struct netcfg_t {
    src:            c_int,
    method:         c_int,
    ip:             u32,
    gateway:        u32,
    netmask:        u32,
    broadcast:      u32,
    dns:            [u32; 2],
    hostname:       [c_char; 64],
    email:          [c_char; 64],
    smtp:           [c_char; 64],
    pop3:           [c_char; 64],
    pop3_login:     [c_char; 64],
    pop3_passwd:    [c_char; 64],
    proxy_host:     [c_char; 64],
    proxy_port:     c_int,
    ppp_login:      [c_char; 64],
    ppp_passwd:     [c_char; 64],
    driver:         [c_char; 64],
}

extern "C" {
    pub fn netcfg_load_from(r#fn: *const c_char, out: *mut netcfg_t) -> c_int;
    pub fn netcfg_load_flash(out: *mut netcfg_t) -> c_int;
    pub fn netcfg_load(out: *mut netcfg_t) -> c_int;
    pub fn netcfg_save_to(r#fn: *const c_char, cfg: *const netcfg_t) -> c_int;
    pub fn netcfg_save(cfg: *const netcfg_t) -> c_int;
}
