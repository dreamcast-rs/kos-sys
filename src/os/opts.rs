//FIXME: Parse these options on the fly from C opts.h.
//       For now, we'll hardcode the default values.
//       FD_SETSIZE is also hardcoded in the libc crate.

pub const KOS_DEBUG: u32                = 0;

pub const FS_CD_MAX_FILES: usize        = 8;
pub const FS_ROMDISK_MAX_FILES: usize   = 16;
pub const FS_RAMDISK_MAX_FILES: usize   = 8;
pub const FD_SETSIZE: usize             = 1024;
