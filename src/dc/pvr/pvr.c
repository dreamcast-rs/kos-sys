// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

#include <dc/pvr.h>

void pvr_set_pal_entry_wrapper(uint32_t idx, uint32_t value) {
    return pvr_set_pal_entry(idx, value);
}

void pvr_dr_commit_wrapper(void *addr) {
    return pvr_dr_commit(addr);
}
