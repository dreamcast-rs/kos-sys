// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub mod controller;
pub mod dreameye;
pub mod keyboard;
pub mod lightgun;
pub mod mouse;
pub mod purupuru;
pub mod sip;
pub mod vmu;

pub const MAPLE_DMA_DEBUG: c_uint           = 0;
pub const MAPLE_IRQ_DEBUG: c_uint           = 0;

pub const MAPLE_BASE: c_uint                = 0xA05F6C00;
pub const MAPLE_DMAADDR: c_uint             = MAPLE_BASE + 0x04;
pub const MAPLE_RESET2: c_uint              = MAPLE_BASE + 0x10;
pub const MAPLE_ENABLE: c_uint              = MAPLE_BASE + 0x14;
pub const MAPLE_STATE: c_uint               = MAPLE_BASE + 0x18;
pub const MAPLE_SPEED: c_uint               = MAPLE_BASE + 0x80;
pub const MAPLE_RESET1: c_uint              = MAPLE_BASE + 0x8C;

pub const MAPLE_RESET2_MAGIC: c_uint        = 0;
pub const MAPLE_ENABLE_ENABLED: c_uint      = 1;
pub const MAPLE_ENABLE_DISABLED: c_uint     = 0;
pub const MAPLE_STATE_IDLE: c_uint          = 0;
pub const MAPLE_STATE_DMA: c_uint           = 1;
pub const MAPLE_SPEED_2MBPS: c_uint         = 0;
#[inline]
pub const fn MAPLE_SPEED_TIMEOUT(n: c_uint) -> c_uint {
    n << 16
}

pub const MAPLE_RESET1_MAGIC: c_uint        = 0x6155405F;

pub const MAPLE_RESPONSE_FILEERR: i8        = -5;
pub const MAPLE_RESPONSE_AGAIN: i8          = -4;
pub const MAPLE_RESPONSE_BADCMD: i8         = -3;
pub const MAPLE_RESPONSE_BADFUNC: i8        = -2;
pub const MAPLE_RESPONSE_NONE: i8           = -1;
pub const MAPLE_COMMAND_DEVINFO: c_int      = 1;
pub const MAPLE_COMMAND_ALLINFO: c_int      = 2;
pub const MAPLE_COMMAND_RESET: c_int        = 3;
pub const MAPLE_COMMAND_KILL: c_int         = 4;
pub const MAPLE_RESPONSE_DEVINFO: i8        = 5;
pub const MAPLE_RESPONSE_ALLINFO: i8        = 6;
pub const MAPLE_RESPONSE_OK: i8             = 7;
pub const MAPLE_RESPONSE_DATATRF: i8        = 8;
pub const MAPLE_COMMAND_GETCOND: c_int      = 9;
pub const MAPLE_COMMAND_GETMINFO: c_int     = 10;
pub const MAPLE_COMMAND_BREAD: c_int        = 11;
pub const MAPLE_COMMAND_BWRITE: c_int       = 12;
pub const MAPLE_COMMAND_BSYNC: c_int        = 13;
pub const MAPLE_COMMAND_SETCOND: c_int      = 14;
pub const MAPLE_COMMAND_MICCONTROL: c_int   = 15;
pub const MAPLE_COMMAND_CAMCONTROL: c_int   = 17;

pub const MAPLE_FUNC_PURUPURU: u32          = 0x00010000;
pub const MAPLE_FUNC_MOUSE: u32             = 0x00020000;
pub const MAPLE_FUNC_CAMERA: u32            = 0x00080000;
pub const MAPLE_FUNC_CONTROLLER: u32        = 0x01000000;
pub const MAPLE_FUNC_MEMCARD: u32           = 0x02000000;
pub const MAPLE_FUNC_LCD: u32               = 0x04000000;
pub const MAPLE_FUNC_CLOCK: u32             = 0x08000000;
pub const MAPLE_FUNC_MICROPHONE: u32        = 0x10000000;
pub const MAPLE_FUNC_ARGUN: u32             = 0x20000000;
pub const MAPLE_FUNC_KEYBOARD: u32          = 0x40000000;
pub const MAPLE_FUNC_LIGHTGUN: u32          = 0x80000000;

#[repr(C)]
pub struct maple_frame_queue {
    pub tqh_first:              *mut maple_frame_t,
    pub tqh_last:               *mut *mut maple_frame_t,
}

#[repr(C)]
pub struct maple_driver_list {
    pub lh_first:               *mut maple_driver_t,
}

#[repr(C)]
pub struct maple_frame_t {
    pub frame_next:             *mut maple_frame_t,
    pub frame_prev:             *mut *mut maple_frame_t,
    pub cmd:                    c_int,
    pub dst_port:               c_int,
    pub dst_unit:               c_int,
    pub length:                 c_int,
    pub state:                  *mut c_int, // volatile
    pub queued:                 *mut c_int, // volatile
    pub send_buf:               *mut c_void,
    pub recv_buf:               *mut u8,
    pub dev:                    *mut maple_device_t,
    pub callback:               Option<unsafe extern "C" fn(*mut maple_state_t,
                                                            *mut maple_frame_t)>,
    pub recv_buf_arr:           [u8; 1024 + 32],
}

pub const MAPLE_FRAME_VACANT: c_int         = 0;
pub const MAPLE_FRAME_UNSENT: c_int         = 1;
pub const MAPLE_FRAME_SENT: c_int           = 2;
pub const MAPLE_FRAME_RESPONDED: c_int      = 3;

#[repr(C)]
pub struct maple_devinfo_t {
    pub functions:              u32,
    pub function_data:          [u32; 3],
    pub area_code:              u8,
    pub connector_direction:    u8,
    pub product_name:           [c_char; 30],
    pub product_license:        [c_char; 60],
    pub standby_power:          u16,
    pub max_power:              u16,
}

#[repr(C)]
pub struct maple_response_t {
    pub response:               i8,
    pub dst_addr:               u8,
    pub src_addr:               u8,
    pub data_len:               u8,
    pub data:                   FAM<u8>,
}

#[repr(C)]
pub struct maple_device_t {
    pub port:                   c_int,
    pub unit:                   c_int,
    pub info:                   maple_devinfo_t,
    pub frame:                  maple_frame_t,
    pub drv:                    *mut maple_driver_t,
    pub probe_mask:             u8,
    pub dev_mask:               u8,
    pub status_valid:           u8, // volatile
    pub status:                 FAM<u32>,
}

pub const MAPLE_PORT_COUNT: c_size_t        = 4;
pub const MAPLE_UNIT_COUNT: c_size_t        = 6;

#[repr(C)]
pub struct maple_port_t {
    pub port:                   c_int,
    pub units:                  [*mut maple_device_t; MAPLE_UNIT_COUNT],
}

#[repr(C)]
pub struct maple_driver_t {
    pub next:                   *mut maple_driver_t,
    pub prev:                   *mut *mut maple_driver_t,
    pub functions:              u32,
    pub name:                   *const c_char,
    pub status_size:            c_size_t,
    pub periodic:               Option<unsafe extern "C" fn(drv: *mut maple_driver_t)>,
    pub attach:                 Option<unsafe extern "C" fn(drv: *mut maple_driver_t,
                                                            dev: *mut maple_device_t)
                                                            -> c_int>,
    pub detach:                 Option<unsafe extern "C" fn(drv: *mut maple_driver_t,
                                                            dev: *mut maple_device_t)>,
}

#[repr(C)]
pub struct maple_state_t {
    pub driver_list:            maple_driver_list,
    pub frame_queue:            maple_frame_queue,
    pub ports:                  [maple_port_t; MAPLE_PORT_COUNT],
    pub dma_cntr:               c_int, // volatile
    pub vbl_cntr:               c_int, // volatile
    pub dma_buffer:             *mut u8,
    pub dma_in_progress:        c_int, // volatile
    pub detect_port_next:       u8,
    pub scan_ready_mask:        u8, // volatile
    pub vbl_handle:             c_int,
    pub gun_port:               c_int,
    pub gun_x:                  c_int,
    pub gun_y:                  c_int,
}

pub const MAPLE_DMA_SIZE: c_size_t          = 16384;

#[macro_export]
macro_rules! maple_read {
    ($addr:expr) => {
        *(($addr as *const u32))
    };
}

#[macro_export]
macro_rules! maple_write {
    ($addr:expr, $value:expr) => {
        *(($addr as *mut u32)) = $value
    };
}

pub const MAPLE_EOK: c_int                  = 0;
pub const MAPLE_EFAIL: c_int                = -1;
pub const MAPLE_EAGAIN: c_int               = -2;
pub const MAPLE_EINVALID: c_int             = -3;
pub const MAPLE_ENOTSUPP: c_int             = -4;
pub const MAPLE_ETIMEOUT: c_int             = -5;

pub type maple_attach_callback_t = Option<unsafe extern "C" fn(*mut maple_device_t)>;
pub type maple_detach_callback_t = Option<unsafe extern "C" fn(*mut maple_device_t)>;

// Not a direct replacement for MAPLE_FOREACH_BEGIN and MAPLE_FOREACH_END,
// but close enough to replicate the functionality used by them
#[macro_export]
macro_rules! MAPLE_FOREACH {
    ($func:expr, $type:ty, $varname:ident, $closure:expr) => {
        for __i in 0.. {
            let __dev = $crate::dc::maple::maple_enum_type(__i, $func);
            if __dev.is_null() {
                break;
            }
            let $varname = $crate::dc::maple::maple_dev_status(__dev) as *mut $type;
            $closure();
        }
    };
}

#[link(name = "kallisti")]
extern "C" {
    pub static mut maple_state: maple_state_t;

    // maple_utils
    pub fn maple_bus_enable();
    pub fn maple_bus_disable();
    pub fn maple_dma_start();
    pub fn maple_dma_stop();
    pub fn maple_dma_in_progress() -> c_int;
    pub fn maple_dma_addr(ptr: *mut c_void);
    pub fn maple_addr(port: c_int, unit: c_int) -> u8;
    pub fn maple_raddr(addr: u8, port: *mut c_int, unit: *mut c_int);
    pub fn maple_pcaps(functions: u32) -> *const c_char;
    pub fn maple_perror(response: c_int) -> *const c_char;
    pub fn maple_dev_valid(p: c_int, u: c_int) -> c_int;
    pub fn maple_gun_enable(port: c_int) -> c_int;
    pub fn maple_gun_disable();
    pub fn maple_gun_read_pos(x: *mut c_int, y: *mut c_int);

    // maple_queue
    pub fn maple_queue_flush();
    pub fn maple_queue_frame(frame: *mut maple_frame_t) -> c_int;
    pub fn maple_queue_remove(frame: *mut maple_frame_t) -> c_int;
    pub fn maple_frame_init(frame: *mut maple_frame_t);
    pub fn maple_frame_lock(frame: *mut maple_frame_t) -> c_int;
    pub fn maple_frame_unlock(frame: *mut maple_frame_t);

    // maple_driver
    pub fn maple_driver_reg(driver: *mut maple_driver_t) -> c_int;
    pub fn maple_driver_unreg(driver: *mut maple_driver_t) -> c_int;
    pub fn maple_driver_attach(det: *mut maple_frame_t) -> c_int;
    pub fn maple_driver_detach(p: c_int, u: c_int) -> c_int;
    pub fn maple_driver_foreach(drv: *mut maple_driver_t,
                                callback: Option<unsafe extern "C" fn(*mut maple_device_t)
                                -> c_int>) -> c_int;
    pub fn maple_attach_callback(functions: u32, cb: maple_attach_callback_t);
    pub fn maple_detach_callback(functions: u32, cb: maple_detach_callback_t);

    // maple_irq
    pub fn maple_vbl_irq_hnd(code: u32, data: *mut c_void);
    pub fn maple_dma_irq_hnd(code: u32, data: *mut c_void);

    // maple_enum
    pub fn maple_enum_count() -> c_int;
    pub fn maple_enum_dev(p: c_int, u: c_int) -> *mut maple_device_t;
    pub fn maple_enum_type(n: c_int, func: u32) -> *mut maple_device_t;
    pub fn maple_enum_type_ex(n: c_int, func: u32, cap: u32) -> *mut maple_device_t;
    pub fn maple_dev_status(dev: *mut maple_device_t) -> *mut c_void;

    // maple_init
    pub fn maple_init();
    pub fn maple_shutdown();
    pub fn maple_wait_scan();
}
