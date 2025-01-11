// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

#include <dc/g2bus.h>

g2_ctx_t g2_lock_wrapper(void) {
    return g2_lock();
}

void g2_unlock_wrapper(g2_ctx_t ctx) {
    return g2_unlock(ctx);
}

