// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

use crate::os::malloc::free;
use crate::os::stdlib::posix_memalign;

extern crate alloc;
use alloc::alloc::{GlobalAlloc, Layout};

struct KOSAllocator;

unsafe impl GlobalAlloc for KOSAllocator {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut out = null_mut();
        let align = layout.align().max(size_of::<usize>());
        let ret = unsafe { posix_memalign(&mut out, align, layout.size()) };
        if ret != 0 { null_mut() } else { out as *mut u8 }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { free(ptr as *mut c_void) };
    }
}

#[global_allocator]
static ALLOCATOR: KOSAllocator = KOSAllocator;
