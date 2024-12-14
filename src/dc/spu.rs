use crate::prelude::*;

pub const SPU_RAM_BASE: c_uintptr_t             = 0x00800000;
pub const SPU_RAM_UNCACHED_BASE: c_uintptr_t    = crate::arch::memory::MEM_AREA_P2_BASE
                                                  as usize | SPU_RAM_BASE as usize;

pub type spu_dma_callback_t = crate::dc::g2bus::g2_dma_callback_t;

extern "C" {
    pub fn spu_memload(to: c_uintptr_t, from: *mut c_void, length: c_size_t);
    pub fn spu_memload_sq(to: c_uintptr_t, from: *mut c_void, length: c_size_t);
    pub fn spu_memread(to: *mut c_void, from: c_uintptr_t, length: c_size_t);
    pub fn spu_memset(to: c_uintptr_t, what: u32, length: c_size_t);
    pub fn spu_memset_sq(to: c_uintptr_t, what: u32, length: c_size_t);
    pub fn spu_dma_transfer(from: *mut c_void, dest: c_uintptr_t, length: c_size_t,
                            block: c_int, callback: spu_dma_callback_t,
                            cbdata: *mut c_void) -> c_int;
    pub fn spu_enable();
    pub fn spu_disable();
    pub fn spu_cdda_volume(left_volume: c_int, right_volume: c_int);
    pub fn spu_cdda_pan(left_pan: c_int, right_pan: c_int);
    pub fn spu_master_mixer(volume: c_int, stereo: c_int);
    pub fn spu_init() -> c_int;
    pub fn spu_shutdown() -> c_int;
    pub fn spu_reset_chans();
}
