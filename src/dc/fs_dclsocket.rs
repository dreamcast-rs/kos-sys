use crate::prelude::*;

extern "C" {
    pub static mut dbgio_dcls: crate::os::dbgio::dbgio_handler_t;
    pub fn fs_dclsocket_init_console();
    pub fn fs_dclsocket_init() -> c_int;
    pub fn fs_dclsocket_shutdown();
}
