// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

use super::perfctr::perf_cntr_event_t;

#[repr(C)]
pub struct perf_monitor {
    r#fn:           *const c_char,
    line:           c_uint,
    calls:          u64,
    time_ns:        u64,
    time_start:     u64,
    event0:         u64,
    event0_start:   u64,
    event1:         u64,
    event1_start:   u64,
}

// FIXME: Unimplemented bindings for the following:

/*
    #define __perf_monitor(f, l)
    #define _perf_monitor(f, l)
    #define __perf_monitor_if(f, l, tst)
    #define _perf_monitor_if(f, l, tst)
    #define perf_monitor()
    #define perf_monitor_if(tst)

*/

#[link(name = "kallisti")]
extern "C" {
    pub fn __stop_perf_monitor(monitor: *mut *mut perf_monitor);
    pub fn __start_perf_monitor(monitor: *mut perf_monitor) -> *mut perf_monitor;
    pub fn perf_monitor_init(event1: perf_cntr_event_t, event2: perf_cntr_event_t);
    pub fn perf_monitor_exit();
    pub fn perf_monitor_print(f: *mut FILE);
}
