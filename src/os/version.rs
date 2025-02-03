// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub const KOS_VERSION_MAJOR: u8     = 2;
pub const KOS_VERSION_MINOR: u16    = 1;
pub const KOS_VERSION_PATCH: u8     = 0;

#[inline]
pub const fn KOS_VERSION_MAKE(major: u8, minor: u16, patch: u8) -> kos_version_t {
    ((major as u32) << 16) | ((minor as u32) << 8) | (patch as u32)
}

pub const KOS_VERSION: kos_version_t = KOS_VERSION_MAKE(KOS_VERSION_MAJOR, KOS_VERSION_MINOR, KOS_VERSION_PATCH);

#[inline]
pub const fn KOS_VERSION_STRING() -> &'static str {
    concat!(
        stringify!(KOS_VERSION_MAJOR),
        ".",
        stringify!(KOS_VERSION_MINOR),
        ".",
        stringify!(KOS_VERSION_PATCH)
    )
}

#[inline]
pub const fn KOS_VERSION_ABOVE(major: u8, minor: u16, patch: u8) -> bool {
    KOS_VERSION_MAKE(major, minor, patch) < KOS_VERSION
}

#[inline]
pub const fn KOS_VERSION_MIN(major: u8, minor: u16, patch: u8) -> bool {
    KOS_VERSION_MAKE(major, minor, patch) <= KOS_VERSION
}

#[inline]
pub const fn KOS_VERSION_IS(major: u8, minor: u16, patch: u8) -> bool {
    KOS_VERSION_MAKE(major, minor, patch) == KOS_VERSION
}

#[inline]
pub const fn KOS_VERSION_MAX(major: u8, minor: u16, patch: u8) -> bool {
    KOS_VERSION_MAKE(major, minor, patch) >= KOS_VERSION
}

#[inline]
pub const fn KOS_VERSION_BELOW(major: u8, minor: u16, patch: u8) -> bool {
    KOS_VERSION_MAKE(major, minor, patch) > KOS_VERSION
}

pub type kos_version_t = u32;

#[link(name = "kallisti")]
extern "C" {
    pub fn kos_version() -> kos_version_t;
    pub fn kos_version_string() -> *const c_char;
    pub fn kos_version_above(major: u8, minor: u16, patch: u8) -> bool;
    pub fn kos_version_min(major: u8, minor: u16, patch: u8) -> bool;
    pub fn kos_version_is(major: u8, minor: u16, patch: u8) -> bool;
    pub fn kos_version_max(major: u8, minor: u16, patch: u8) -> bool;
    pub fn kos_version_below(major: u8, minor: u16, patch: u8) -> bool;
}
