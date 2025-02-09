// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

/* Here we re-export extremely common types and
   functions for interoperating with KOS C code */

pub use core::{
    ffi::{
        c_char,
        c_schar,
        c_uchar,
        c_short,
        c_ushort,
        c_int,
        c_uint,
        c_long,
        c_ulong,
        c_size_t,
        c_ssize_t,
        c_longlong,
        c_ulonglong,
        c_ptrdiff_t,
        c_float,
        c_double,
        c_void,
        CStr,
        VaList,
    },
    mem::{
        size_of,
    },
    ptr::{
        null,
        null_mut,
        read_volatile,
        write_volatile,
    },
};

pub type c_uintptr_t = usize;

pub use crate::os::types::{
    off_t,
    off64_t,
    pid_t,
    time_t,
};

pub use libc::{
    DIR,
    FILE,
    stat,
    timespec,
    timeval,
};

#[repr(C)]
#[derive(Default)]
pub struct FAM<T>(core::marker::PhantomData<T>, [T; 0]);
impl<T> FAM<T> {
    #[inline]
    pub const fn new() -> Self {
        FAM(core::marker::PhantomData, [])
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }

    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.as_ptr(), len) }
    }

    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self.as_mut_ptr(), len) }
    }
}

impl<T> core::fmt::Debug for FAM<T> {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt.write_str("FAM")
    }
}
