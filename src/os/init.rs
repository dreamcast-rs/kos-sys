// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[macro_export]
macro_rules! KOS_INIT_FLAGS {
    ($flags:expr) => {
        use $crate::{arch::init_flags::*, os::init::*};
        #[unsafe(no_mangle)]
        pub static __kos_init_flags: u32 = $flags;
        $crate::KOS_INIT_FLAG!($flags, $crate::os::init::INIT_NET, arch_init_net);
        $crate::KOS_INIT_FLAG!($flags, $crate::os::init::INIT_NET, net_shutdown);
        $crate::KOS_INIT_FLAG!($flags, $crate::os::init::INIT_NET, bba_la_init);
        $crate::KOS_INIT_FLAG!($flags, $crate::os::init::INIT_NET, bba_la_shutdown);
        $crate::KOS_INIT_FLAG!($flags, $crate::os::init::INIT_FS_ROMDISK,
                               fs_romdisk_init);
        $crate::KOS_INIT_FLAG!($flags, $crate::os::init::INIT_FS_ROMDISK,
                               fs_romdisk_shutdown);
        $crate::KOS_INIT_FLAG!($flags, $crate::os::init::INIT_FS_NULL, fs_null_init);
        $crate::KOS_INIT_FLAG!($flags, $crate::os::init::INIT_FS_NULL, fs_null_shutdown);
        $crate::KOS_INIT_FLAG!($flags, $crate::os::init::INIT_FS_PTY, fs_pty_init);
        $crate::KOS_INIT_FLAG!($flags, $crate::os::init::INIT_FS_PTY, fs_pty_shutdown);
        $crate::KOS_INIT_FLAG!($flags, $crate::os::init::INIT_FS_RAMDISK, fs_ramdisk_init);
        $crate::KOS_INIT_FLAG!($flags, $crate::os::init::INIT_FS_RAMDISK,
                               fs_ramdisk_shutdown);
        $crate::KOS_INIT_FLAG!($flags, $crate::os::init::INIT_FS_RND, fs_rnd_init);
        $crate::KOS_INIT_FLAG!($flags, $crate::os::init::INIT_FS_RND, fs_rnd_shutdown);
        $crate::KOS_INIT_FLAG!($flags, $crate::os::init::INIT_FS_DEV, fs_dev_init);
        $crate::KOS_INIT_FLAG!($flags, $crate::os::init::INIT_FS_DEV, fs_dev_shutdown);
        $crate::KOS_INIT_FLAG!($flags, $crate::os::init::INIT_EXPORT, export_init);
        $crate::KOS_INIT_FLAG!($flags, $crate::os::init::INIT_LIBRARY, library_init);
        $crate::KOS_INIT_FLAG!($flags, $crate::os::init::INIT_LIBRARY, library_shutdown);
        $crate::KOS_INIT_FLAG_NONE!($flags, $crate::os::init::INIT_NO_SHUTDOWN,
                                    kos_shutdown);
        $crate::KOS_INIT_FLAGS_ARCH!($flags);
    };
}

#[macro_export]
macro_rules! KOS_INIT_EARLY {
    ($func:expr) => {
        #[unsafe(no_mangle)]
        pub static __kos_init_early_fn: Option<unsafe extern "C" fn()> = Some($func);
    };
}

pub const INIT_DEFAULT: u32     = INIT_IRQ | INIT_THD_PREEMPT | INIT_FS_ALL | INIT_LIBRARY |
                                  crate::arch::init_flags::INIT_DEFAULT_ARCH;

pub const INIT_FS_DEV: u32      = INIT_FS_NULL | INIT_FS_RND;

pub const INIT_FS_ALL: u32      = INIT_FS_ROMDISK | INIT_FS_RAMDISK |
                                  INIT_FS_PTY | INIT_FS_DEV;

pub const INIT_NONE: u32        = 0x00000000;
pub const INIT_THD_PREEMPT: u32 = 0x00000000;
pub const INIT_IRQ: u32         = 0x00000001;
pub const INIT_NET: u32         = 0x00000002;
pub const INIT_MALLOCSTATS: u32 = 0x00000004;
pub const INIT_QUIET: u32       = 0x00000008;
pub const INIT_EXPORT: u32      = 0x00000010;
pub const INIT_LIBRARY: u32     = 0x00000010;

pub const INIT_FS_ROMDISK: u32  = 0x00000020;
pub const INIT_FS_RAMDISK: u32  = 0x00000040;
pub const INIT_FS_PTY: u32      = 0x00000080;
pub const INIT_FS_NULL: u32     = 0x00000100;
pub const INIT_FS_RND: u32      = 0x00000200;

pub const INIT_NO_SHUTDOWN: u32 = 0x00000400;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub static __kos_romdisk: *mut c_void;
}
