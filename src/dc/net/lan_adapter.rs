// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[link(name = "kallisti")]
extern "C" {
   pub fn la_init() -> c_int;
   pub fn la_shutdown() -> c_int;
}
