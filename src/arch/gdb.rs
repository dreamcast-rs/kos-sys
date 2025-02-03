// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn gdb_init();
    pub fn gdb_breakpoint();
}
