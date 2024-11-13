use crate::prelude::*;

pub const FS_EXT2_MOUNT_READONLY: u32   = 0x00000000;
pub const FS_EXT2_MOUNT_READWRITE: u32  = 0x00000001;

extern "C" {
    pub fn fs_ext2_init() -> c_int;
    pub fn fs_ext2_shutdown() -> c_int;
    pub fn fs_ext2_mount(mp: *const c_char, dev: *mut kos_blockdev_t,
                         flags: u32) -> c_int;
    pub fn fs_ext2_unmount(mp: *const c_char) -> c_int;
    pub fn fs_ext2_sync(mp: *const c_char) -> c_int;
}

