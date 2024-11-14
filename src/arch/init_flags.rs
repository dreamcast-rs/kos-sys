#[macro_export]
macro_rules! KOS_INIT_FLAGS_ARCH {
    ($flags:expr) => {
        $crate::KOS_INIT_FLAG_NONE!($flags, $crate::arch::init_flags::INIT_NO_DCLOAD,
                                    dcload_init);
        $crate::KOS_INIT_FLAG_NONE!($flags, $crate::arch::init_flags::INIT_NO_DCLOAD,
                                    fs_dcload_init_console);
        $crate::KOS_INIT_FLAG_NONE!($flags, $crate::arch::init_flags::INIT_NO_DCLOAD,
                                    fs_dcload_shutdown);
        $crate::KOS_INIT_FLAG_NONE!($flags, $crate::arch::init_flags::INIT_NO_DCLOAD,
                                    arch_init_net_dcload_ip);
        $crate::KOS_INIT_FLAG!($flags, $crate::arch::init_flags::INIT_NO_DCLOAD,
                               arch_init_net_no_dcload);
        $crate::KOS_INIT_FLAG!($flags, $crate::arch::init_flags::INIT_CDROM,
                               cdrom_init);
        $crate::KOS_INIT_FLAG!($flags, $crate::arch::init_flags::INIT_CDROM,
                               cdrom_shutdown);
        $crate::KOS_INIT_FLAG!($flags, $crate::arch::init_flags::INIT_CDROM,
                               fs_iso9660_init);
        $crate::KOS_INIT_FLAG!($flags, $crate::arch::init_flags::INIT_CDROM,
                               fs_iso9660_shutdown);
        $crate::KOS_INIT_FLAG!($flags, $crate::arch::init_flags::INIT_CONTROLLER,
                               cont_init);
        $crate::KOS_INIT_FLAG!($flags, $crate::arch::init_flags::INIT_CONTROLLER,
                               cont_shutdown);
        $crate::KOS_INIT_FLAG!($flags, $crate::arch::init_flags::INIT_KEYBOARD,
                               kbd_init);
        $crate::KOS_INIT_FLAG!($flags, $crate::arch::init_flags::INIT_KEYBOARD,
                               kbd_shutdown);
        $crate::KOS_INIT_FLAG!($flags, $crate::arch::init_flags::INIT_MOUSE,
                               mouse_init);
        $crate::KOS_INIT_FLAG!($flags, $crate::arch::init_flags::INIT_MOUSE,
                               mouse_shutdown);
        $crate::KOS_INIT_FLAG!($flags, $crate::arch::init_flags::INIT_LIGHTGUN,
                               lightgun_init);
        $crate::KOS_INIT_FLAG!($flags, $crate::arch::init_flags::INIT_LIGHTGUN, 
                               lightgun_shutdown);
        $crate::KOS_INIT_FLAG!($flags, $crate::arch::init_flags::INIT_VMU,
                               vmu_init);
        $crate::KOS_INIT_FLAG!($flags, $crate::arch::init_flags::INIT_VMU,
                               vmu_shutdown);
        $crate::KOS_INIT_FLAG!($flags, $crate::arch::init_flags::INIT_VMU,
                               vmu_fs_init);
        $crate::KOS_INIT_FLAG!($flags, $crate::arch::init_flags::INIT_VMU,
                               vmu_fs_shutdown);
        $crate::KOS_INIT_FLAG!($flags, $crate::arch::init_flags::INIT_PURUPURU,
                               purupuru_init);
        $crate::KOS_INIT_FLAG!($flags, $crate::arch::init_flags::INIT_PURUPURU,
                               purupuru_shutdown);
        $crate::KOS_INIT_FLAG!($flags, $crate::arch::init_flags::INIT_SIP,
                               sip_init);
        $crate::KOS_INIT_FLAG!($flags, $crate::arch::init_flags::INIT_SIP,
                               sip_shutdown);
        $crate::KOS_INIT_FLAG!($flags, $crate::arch::init_flags::INIT_DREAMEYE,
                               dreameye_init);
        $crate::KOS_INIT_FLAG!($flags, $crate::arch::init_flags::INIT_DREAMEYE,
                               dreameye_shutdown);
        $crate::KOS_INIT_FLAG!($flags, $crate::arch::init_flags::INIT_MAPLE_ALL,
                               maple_wait_scan);
        $crate::KOS_INIT_FLAG!($flags, $crate::arch::init_flags::INIT_MAPLE_ALL,
                               maple_init);
        $crate::KOS_INIT_FLAG!($flags, $crate::arch::init_flags::INIT_MAPLE_ALL,
                               maple_shutdown);
    };
}

pub const INIT_DEFAULT_ARCH: u32    = INIT_MAPLE_ALL | INIT_CDROM;

pub const INIT_CONTROLLER: u32      = 0x00001000;
pub const INIT_KEYBOARD: u32        = 0x00002000;
pub const INIT_MOUSE: u32           = 0x00004000;
pub const INIT_LIGHTGUN: u32        = 0x00008000;
pub const INIT_VMU: u32             = 0x00010000;
pub const INIT_PURUPURU: u32        = 0x00020000;
pub const INIT_SIP: u32             = 0x00040000;
pub const INIT_DREAMEYE: u32        = 0x00080000;
pub const INIT_MAPLE_ALL: u32       = 0x000FF000;

pub const INIT_CDROM: u32           = 0x00100000;

pub const INIT_OCRAM: u32           = 0x10000000;
pub const INIT_NO_DCLOAD: u32       = 0x20000000;