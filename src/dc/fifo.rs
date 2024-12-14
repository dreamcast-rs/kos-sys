use core::ptr::read_volatile;

#[inline]
pub fn FIFO_STATUS() -> u32 {
    unsafe {
        read_volatile(0xa05f688c as *const u32)
    }
}

pub const FIFO_AICA: u32    = 1 << 0;
pub const FIFO_BBA: u32     = 1 << 1;
pub const FIFO_EXT2: u32    = 1 << 2;
pub const FIFO_EXTDEV: u32  = 1 << 3;
pub const FIFO_G2: u32      = 1 << 4;
pub const FIFO_SH4: u32     = 1 << 5;
