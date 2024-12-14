use crate::prelude::*;

#[repr(C)]
pub struct dbgio_handler_t {
    name:           *const c_char,
    detected:       Option<unsafe extern "C" fn() -> c_int>,
    init:           Option<unsafe extern "C" fn() -> c_int>,
    shutdown:       Option<unsafe extern "C" fn() -> c_int>,
    set_irq_usage:  Option<unsafe extern "C" fn(mode: c_int) -> c_int>,
    read:           Option<unsafe extern "C" fn() -> c_int>,
    write:          Option<unsafe extern "C" fn(c: c_int) -> c_int>,
    flush:          Option<unsafe extern "C" fn() -> c_int>,
    write_buffer:   Option<unsafe extern "C" fn(data: *const u8, len: c_int,
                                                xlat: c_int) -> c_int>,
    read_buffer:    Option<unsafe extern "C" fn(data: *mut u8, len: c_int) -> c_int>,
}

pub const DBGIO_MODE_POLLED: c_int  = 0;
pub const DBGIO_MODE_IRQ: c_int     = 1;

extern "C" {
    pub static dbgio_handlers: *mut *mut dbgio_handler_t;
    pub static dbgio_handler_cnt: c_int;
    pub static dbgio_null: dbgio_handler_t;
    pub fn dbgio_dev_select(name: *const c_char) -> c_int;
    pub fn dbgio_dev_get() -> *const c_char;
    pub fn dbgio_init() -> c_int;
    pub fn dbgio_set_irq_usage(mode: c_int) -> c_int;
    pub fn dbgio_read() -> c_int;
    pub fn dbgio_write(c: c_int) -> c_int;
    pub fn dbgio_flush() -> c_int;
    pub fn dbgio_write_buffer(data: *const u8, len: c_int) -> c_int;
    pub fn dbgio_read_buffer(data: *mut u8, len: c_int) -> c_int;
    pub fn dbgio_write_buffer_xlat(data: *const u8, len: c_int) -> c_int;
    pub fn dbgio_write_str(str: *const c_char) -> c_int;
    pub fn dbgio_disable();
    pub fn dbgio_enable();
    pub fn dbgio_printf(fmt: *const c_char, ...) -> c_int;
}
