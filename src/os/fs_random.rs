// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn fs_rnd_init();
    pub fn fs_rnd_shutdown();
}
