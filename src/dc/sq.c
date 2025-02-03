// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

#include <dc/sq.h>

void sq_flush_wrapper(void *dest) {
    sq_flush(dest);
}
