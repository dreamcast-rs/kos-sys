// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

pub use libc::{
    c_char,
    c_float,
    c_int,
    c_long,
    c_longlong,
    c_short,
    c_size_t,
    c_ssize_t,
    c_uchar,
    c_uint,
    c_uintptr_t,
    c_ulong,
    c_ushort,
    c_void,
};

use core::fmt::{Debug, Formatter, Result};
use core::marker::PhantomData;
use core::slice;

#[repr(C)]
#[derive(Default)]
pub struct FAM<T>(PhantomData<T>, [T; 0]);
impl<T> FAM<T> {
    #[inline]
    pub const fn new() -> Self {
        FAM(PhantomData, [])
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
        slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}

impl<T> Debug for FAM<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        fmt.write_str("FAM")
    }
}
