// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn fs_dev_init();
    pub fn fs_dev_shutdown();
}
