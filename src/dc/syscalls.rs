use crate::prelude::*;

extern "C" {
    pub fn syscalls_sysinfo_init();
    pub fn syscall_sysinfo_icon(icon: u32, dest: *mut u8) -> c_int;
    pub fn syscall_sysinfo_id() -> u64;
    pub fn syscall_font_address() -> *mut u8;
    pub fn syscall_font_lock() -> c_int;
    pub fn syscall_font_unlock();
    pub fn syscall_flashrom_info(part: u32, info: *mut c_void) -> c_int;
    pub fn syscall_flashrom_read(pos: u32, dest: *mut c_void, n: c_size_t) -> c_int;
    pub fn syscall_flashrom_write(pos: u32, src: *const c_void, n: c_size_t) -> c_int;
    pub fn syscall_flashrom_delete(pos: u32) -> c_int;
    pub fn syscall_gdrom_init();
    pub fn syscall_gdrom_reset();
    pub fn syscall_gdrom_check_drive(status: *mut u32) -> c_int;
    pub fn syscall_gdrom_send_command(cmd: u32, params: *mut c_void) -> u32;
    pub fn syscall_gdrom_check_command(id: u32, status: *mut i32) -> c_int;
    pub fn syscall_gdrom_exec_server();
    pub fn syscall_gdrom_abort_command(id: u32) -> c_int;
    pub fn syscall_gdrom_sector_mode(mode: *mut u32) -> c_int;
    pub fn syscall_gdrom_dma_callback(callback: c_uintptr_t, param: *mut c_void);
    pub fn syscall_gdrom_dma_transfer(id: u32, params: *const i32) -> c_int;
    pub fn syscall_gdrom_dma_check(id: u32, size: *mut c_size_t) -> c_int;
    pub fn syscall_gdrom_pio_callback(callback: c_uintptr_t, param: *mut c_void);
    pub fn syscall_gdrom_pio_transfer(id: u32, params: *const i32) -> c_int;
    pub fn syscall_gdrom_pio_check(id: u32, size: *mut c_size_t) -> c_int;
    pub fn syscall_misc_init() -> c_int;
    // "super" renamed to "superfn"
    pub fn syscall_misc_setvector(superfn: u32, handler: c_uintptr_t) -> c_int;
    pub fn syscall_system_reset() -> !;
    pub fn syscall_system_bios_menu() -> !;
    pub fn syscall_system_cd_menu() -> !;
}