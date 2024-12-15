use crate::prelude::*;
use libc::time_t;

extern "C" {
    pub fn rtc_unix_secs() -> time_t;
    pub fn rtc_set_unix_secs(time: time_t) -> c_int;
    pub fn rtc_boot_time() -> time_t;
    pub fn rtc_init() -> c_int;
    pub fn rtc_shutdown();
}
