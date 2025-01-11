// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

#include <dc/sq.h>

void sq_flush_wrapper(const void *dest) {
    sq_flush(dest);
}
