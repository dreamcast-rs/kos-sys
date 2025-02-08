// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

use crate::arch::types::{prio_t, tid_t};
use crate::arch::irq::irq_context_t;

use super::tls::kthread_tls_kv_list;

pub const KOS_PID: pid_t                    = 1;
pub const PRIO_MAX: prio_t                  = 4096;
pub const PRIO_DEFAULT: prio_t              = 10;
pub const KTHREAD_LABEL_SIZE: usize         = 256;
pub const KTHREAD_PWD_SIZE: usize           = 256;

#[repr(C)]
pub struct ktqueue {
    pub tqh_first:      *mut kthread_t,
    pub tqh_last:       *mut *mut kthread_t,
}

#[repr(C)]
pub struct ktlist {
    pub lh_first:       *mut kthread_t,
}

pub const THD_DEFAULTS: kthread_flags_t     = 0;
pub const THD_USER: kthread_flags_t         = 1;
pub const THD_QUEUED: kthread_flags_t       = 2;
pub const THD_DETACHED: kthread_flags_t     = 4;
pub const THD_OWNS_STACK: kthread_flags_t   = 8;

pub type kthread_flags_t = u8;

#[repr(C)]
pub enum kthread_state_t {
    STATE_ZOMBIE    = 0x0000,
    STATE_RUNNING   = 0x0001,
    STATE_READY     = 0x0002,
    STATE_WAIT      = 0x0003,
    STATE_FINISHED  = 0x0004,
}

#[repr(C)]
pub struct tcbhead_t {
    pub dtv:            *mut c_void,
    pub pointer_guard:  c_uintptr_t,
}

// FIXME:
// Treating _reent as an opaque type for now
// 316 bytes on Newlib 4.5.0
#[repr(C)]
pub struct _reent {
    _opaque:            [u8; 316],
}

#[repr(C)]
pub struct t_list {
    pub le_next:        *mut kthread_t,
    pub le_prev:        *mut *mut kthread_t,
}

#[repr(C)]
pub struct thdq {
    pub tqe_next:       *mut kthread_t,
    pub tqe_prev:       *mut *mut kthread_t,
}

#[repr(C)]
pub struct timerq {
    pub tqe_next:       *mut kthread_t,
    pub tqe_prev:       *mut *mut kthread_t,
}

#[repr(C)]
pub struct cpu_time {
    pub scheduled:      u64,
    pub total:          u64,
}

#[repr(C, align(32))]
pub struct kthread_t {
    context:            irq_context_t,
    t_list:             t_list,
    thdq:               thdq,
    timerq:             timerq,
    tid:                tid_t,
    prio:               prio_t,
    real_prio:          prio_t,
    flags:              kthread_flags_t,
    state:              kthread_state_t,
    wait_obj:           *mut c_void,
    wait_msg:           *const c_char,
    wait_callback:      Option<unsafe extern "C" fn(obj: *mut c_void)>,
    wait_timeout:       u64,
    cpu_time:           cpu_time,
    label:              [c_char; KTHREAD_LABEL_SIZE],
    pwd:                [c_char; KTHREAD_PWD_SIZE],
    stack:              *mut c_void,
    stack_size:         c_size_t,
    thd_errno:          c_int,
    thd_reent:          _reent,
    tls_list:           kthread_tls_kv_list,
    tcbhead:            *mut tcbhead_t,
    rv:                 *mut c_void,
}


#[repr(C)]
pub struct kthread_attr_t {
    create_detached:    bool,
    stack_size:         c_size_t,
    stack_ptr:          *mut c_void,
    prio:               prio_t,
    label:              *const c_char,
}

#[repr(C)]
pub enum kthread_mode_t {
    THD_MODE_NONE = -1,
    THD_MODE_COOP = 0,
    THD_MODE_PREEMPT = 1,
}

#[link(name = "kallisti")]
unsafe extern "C" {
    pub static mut thd_current: *mut kthread_t;
    pub fn thd_block_now(mycxt: *mut irq_context_t) -> c_int;
    pub fn thd_choose_new() -> *mut irq_context_t;
    pub fn thd_by_tid(tid: tid_t) -> *mut kthread_t;
    pub fn thd_add_to_runnable(t: *mut kthread_t, front_of_line: bool);
    pub fn thd_remove_from_runnable(thd: *mut kthread_t) -> c_int;
    pub fn thd_create(
        detach: bool,
        routine: Option<unsafe extern "C" fn(param: *mut c_void) -> *mut c_void>,
        param: *mut c_void,
    ) -> *mut kthread_t;
    pub fn thd_create_ex(
        attr: *const kthread_attr_t,
        routine: Option<unsafe extern "C" fn(param: *mut c_void) -> *mut c_void>,
        param: *mut c_void,
    ) -> *mut kthread_t;
    pub fn thd_destroy(thd: *mut kthread_t) -> c_int;
    pub fn thd_exit(rv: *mut c_void) -> !;
    pub fn thd_schedule(front_of_line: bool, now: u64);
    pub fn thd_schedule_next(thd: *mut kthread_t);
    pub fn thd_pass();
    pub fn thd_sleep(ms: c_uint);
    pub fn thd_set_prio(thd: *mut kthread_t, prio: prio_t) -> c_int;
    pub fn thd_get_prio(thd: *mut kthread_t) -> prio_t;
    pub fn thd_get_id(thd: *mut kthread_t) -> tid_t;
    pub fn thd_get_current() -> *mut kthread_t;
    pub fn thd_get_label(thd: *mut kthread_t) -> *const c_char;
    pub fn thd_set_label(thd: *mut kthread_t, label: *const c_char);
    pub fn thd_get_pwd(thd: *mut kthread_t) -> *const c_char;
    pub fn thd_set_pwd(thd: *mut kthread_t, pwd: *const c_char);
    pub fn thd_get_errno(thd: *mut kthread_t) -> *mut c_int;
    pub fn thd_get_reent(thd: *mut kthread_t) -> *mut _reent;
    pub fn thd_get_cpu_time(thd: *mut kthread_t) -> u64;
    pub fn thd_set_hz(hertz: c_uint) -> c_int;
    pub fn thd_get_hz() -> c_uint;
    pub fn thd_join(thd: *mut kthread_t, value_ptr: *mut *mut c_void) -> c_int;
    pub fn thd_detach(thd: *mut kthread_t) -> c_int;
    pub fn thd_each(
        cb: Option<unsafe extern "C" fn(thd: *mut kthread_t, user_data: *mut c_void) -> c_int>,
        data: *mut c_void,
    ) -> c_int;
    pub fn thd_pslist(pf: Option<unsafe extern "C" fn(fmt: *const c_char, ...) -> c_int>) -> c_int;
    pub fn thd_pslist_queue(
        pf: Option<unsafe extern "C" fn(fmt: *const c_char, ...) -> c_int>,
    ) -> c_int;
    pub fn thd_init() -> c_int;
    pub fn thd_shutdown();
}
