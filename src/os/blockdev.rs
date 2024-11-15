use crate::prelude::*;

#[repr(C)]
pub struct kos_blockdev_t {
    pub dev_data:       *mut c_void,
    pub l_block_size:   u32,
    pub init:           Option<unsafe extern "C" fn(d: *mut kos_blockdev_t) -> c_int>,
    pub shutdown:       Option<unsafe extern "C" fn(d: *mut kos_blockdev_t) -> c_int>,
    pub read_blocks:    Option<unsafe extern "C" fn(d: *mut kos_blockdev_t, block: u64,
                                                    count: c_size_t, buf: *mut c_void)
                                                    -> c_int>,
    pub write_blocks:   Option<unsafe extern "C" fn(d: *mut kos_blockdev_t, block: u64,
                                                    count: c_size_t, buf: *const c_void)
                                                    -> c_int>,
    pub count_blocks:   Option<unsafe extern "C" fn(d: *mut kos_blockdev_t) -> u64>,
    pub flush:          Option<unsafe extern "C" fn(d: *mut kos_blockdev_t) -> c_int>,
}
