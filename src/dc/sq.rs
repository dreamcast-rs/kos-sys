use crate::prelude::*;

#[inline]
pub const fn SQ_MASK_DEST_ADDR(dest: c_uintptr_t) -> c_uintptr_t {
    crate::arch::memory::MEM_AREA_SQ_BASE as c_uintptr_t | dest & 0x03ffffe0
}

#[inline]
pub const fn SQ_MASK_DEST(dest: c_uintptr_t) -> *mut u32 {
    SQ_MASK_DEST_ADDR(dest) as *mut u32
}

extern "C" {
    pub fn sq_lock(dest: *mut c_void);
    pub fn sq_unlock();
    pub fn sq_wait();
    #[link_name = "sq_flush_stub"]
    pub fn sq_flush(dest: *mut c_void);
    pub fn sq_cpy(dest: *mut c_void, src: *const c_void, n: c_size_t) -> *mut c_void;
    pub fn sq_fast_cpy(dest: *mut c_void, src: *const c_void, n: c_size_t) -> *mut c_void;
    pub fn sq_set(dest: *mut c_void, c: u32, n: c_size_t) -> *mut c_void;
    pub fn sq_set16(dest: *mut c_void, c: u32, n: c_size_t) -> *mut c_void;
    pub fn sq_set32(dest: *mut c_void, c: u32, n: c_size_t) -> *mut c_void;
    pub fn sq_clr(dest: *mut c_void, n: c_size_t);
}
