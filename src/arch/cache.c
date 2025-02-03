// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

#include <arch/cache.h>

void dcache_pref_block_wrapper(const void *src) {
    dcache_pref_block(src);
}

void dcache_wback_sq_wrapper(void *src) {
    dcache_wback_sq(src);
}

void dcache_alloc_block_wrapper(void *src, uint32_t value) {
    dcache_alloc_block(src, value);
}
