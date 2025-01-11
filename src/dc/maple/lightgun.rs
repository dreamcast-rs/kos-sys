// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

#[link(name = "kallisti")]
extern "C" {
    pub fn lightgun_init();
    pub fn lightgun_shutdown();
}
