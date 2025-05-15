// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

#include <dc/fs_vmu.h>

int fs_vmu_set_header_wrapper(file_t fd, const vmu_pkg_t *pkg) {
    return fs_vmu_set_header(fd, pkg);
}

int fs_vmu_set_default_header_wrapper(const vmu_pkg_t *pkg) {
    return fs_vmu_set_default_header(pkg);
}
