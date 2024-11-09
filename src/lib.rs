#![doc(html_logo_url = "https://kos-sys.dreamcast.wiki/kos-rs_logo.png")]
#![doc(html_favicon_url = "https://kos-sys.dreamcast.wiki/kos-sys_favicon.ico")]
#![no_std]

//! Raw/unsafe bindings to KallistiOS v2.1.0 for Sega Dreamcast.
//!
//! See [dreamcast.rs](https://dreamcast.rs) or the [dreamcast.wiki](https://dreamcast.wiki)
//! for more information on setting up KallistiOS and Rust to use this crate.

pub mod prelude;
extern crate libc;

pub mod kos;
pub mod malloc;
