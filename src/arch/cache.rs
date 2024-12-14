use crate::prelude::*;

pub const CPU_CACHE_BLOCK_SIZE: usize  = 32;

extern "C" {
    pub fn icache_flush_range(start: c_uintptr_t, count: c_size_t);
    pub fn dcache_inval_range(start: c_uintptr_t, count: c_size_t);
    pub fn dcache_flush_range(start: c_uintptr_t, count: c_size_t);
    pub fn dcache_flush_all();
    pub fn dcache_purge_range(start: c_uintptr_t, count: c_size_t);
    pub fn dcache_purge_all();
    pub fn dcache_purge_all_with_buffer(start: c_uintptr_t, count: c_size_t);
    #[link_name = "dcache_pref_block_stub"]
    pub fn dcache_pref_block(src: *const c_void);
    #[link_name = "dcache_wback_sq_stub"]
    pub fn dcache_wback_sq(ptr: *const c_void);
    #[link_name = "dcache_alloc_block_stub"]
    pub fn dcache_alloc_block(src: *const c_void, value: u32);
}