#![allow(non_snake_case)]
use crate::prelude::*;
use core::ptr::write_volatile;

pub const DMAC_BASE: c_uintptr_t        = 0xffa00000;

#[inline]
pub fn DMAC_SAR0(value: u32) {
    unsafe {
        write_volatile((DMAC_BASE + 0x00) as *mut u32, value);
    }
}

#[inline]
pub fn DMAC_SAR1(value: u32) {
    unsafe {
        write_volatile((DMAC_BASE + 0x10) as *mut u32, value);
    }
}

#[inline]
pub fn DMAC_SAR2(value: u32) {
    unsafe {
        write_volatile((DMAC_BASE + 0x20) as *mut u32, value);
    }
}

#[inline]
pub fn DMAC_SAR3(value: u32) {
    unsafe {
        write_volatile((DMAC_BASE + 0x30) as *mut u32, value);
    }
}

#[inline]
pub fn DMAC_DAR0(value: u32) {
    unsafe {
        write_volatile((DMAC_BASE + 0x04) as *mut u32, value);
    }
}

#[inline]
pub fn DMAC_DAR1(value: u32) {
    unsafe {
        write_volatile((DMAC_BASE + 0x14) as *mut u32, value);
    }
}

#[inline]
pub fn DMAC_DAR2(value: u32) {
    unsafe {
        write_volatile((DMAC_BASE + 0x24) as *mut u32, value);
    }
}

#[inline]
pub fn DMAC_DAR3(value: u32) {
    unsafe {
        write_volatile((DMAC_BASE + 0x34) as *mut u32, value);
    }
}

#[inline]
pub fn DMAC_DMATCR0(value: u32) {
    unsafe {
        write_volatile((DMAC_BASE + 0x08) as *mut u32, value);
    }
}

#[inline]
pub fn DMAC_DMATCR1(value: u32) {
    unsafe {
        write_volatile((DMAC_BASE + 0x18) as *mut u32, value);
    }
}

#[inline]
pub fn DMAC_DMATCR2(value: u32) {
    unsafe {
        write_volatile((DMAC_BASE + 0x28) as *mut u32, value);
    }
}

#[inline]
pub fn DMAC_DMATCR3(value: u32) {
    unsafe {
        write_volatile((DMAC_BASE + 0x38) as *mut u32, value);
    }
}

#[inline]
pub fn DMAC_CHCR0(value: u32) {
    unsafe {
        write_volatile((DMAC_BASE + 0x0c) as *mut u32, value);
    }
}

#[inline]
pub fn DMAC_CHCR1(value: u32) {
    unsafe {
        write_volatile((DMAC_BASE + 0x1c) as *mut u32, value);
    }
}

#[inline]
pub fn DMAC_CHCR2(value: u32) {
    unsafe {
        write_volatile((DMAC_BASE + 0x2c) as *mut u32, value);
    }
}

#[inline]
pub fn DMAC_CHCR3(value: u32) {
    unsafe {
        write_volatile((DMAC_BASE + 0x3c) as *mut u32, value);
    }
}

#[inline]
pub fn DMAC_DMAOR(value: u32) {
    unsafe {
        write_volatile((DMAC_BASE + 0x40) as *mut u32, value);
    }
}

pub const DMAOR_STATUS_MASK: u32        = 0x8007;
pub const DMAOR_NORMAL_OPERATION: u32   = 0x8001;

