use crate::prelude::*;

#[repr(C)]
pub struct kos_blockdev_t {
    dev_data:       *mut c_void,
    l_block_size:   u32,
    init:           Option<unsafe extern "C" fn(d: *mut kos_blockdev_t) -> c_int>,
    shutdown:       Option<unsafe extern "C" fn(d: *mut kos_blockdev_t) -> c_int>,
    read_blocks:    Option<unsafe extern "C" fn(d: *mut kos_blockdev_t, block: u64,
                                         count: c_size_t, buf: *mut c_void) -> c_int>,
    write_blocks:   Option<unsafe extern "C" fn(d: *mut kos_blockdev_t, block: u64,
                                                count: c_size_t, buf: *const c_void)
                                                -> c_int>,
    count_blocks:   Option<unsafe extern "C" fn(d: *mut kos_blockdev_t) -> u64>,
    flush:          Option<unsafe extern "C" fn(d: *mut kos_blockdev_t) -> c_int>,
}

