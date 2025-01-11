// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

#![doc(html_logo_url = "https://kos-sys.dreamcast.wiki/kos-rs_logo.png")]
#![doc(html_favicon_url = "https://kos-sys.dreamcast.wiki/kos-sys_favicon.ico")]
#![no_std]

#![feature(c_variadic)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

//! Raw/unsafe bindings to KallistiOS for Sega Dreamcast.
//!
//! See [dreamcast.rs](https://dreamcast.rs) or the [dreamcast.wiki](https://dreamcast.wiki)
//! for more information on setting up KallistiOS and Rust to use this crate.

pub mod prelude;

#[cfg(feature = "allocator")]
pub mod allocator;
#[cfg(feature = "panic_handler")]
pub mod panic_handler;

pub mod addons;
pub mod arch;
pub mod dc;
pub mod os;
pub mod malloc;

// Re-export dependency crates
pub extern crate libc;
pub use paste;
