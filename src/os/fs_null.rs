// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn fs_null_init();
    pub fn fs_null_shutdown();
}
