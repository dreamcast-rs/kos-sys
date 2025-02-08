// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

#include <dc/matrix.h>
#include <dc/vector.h>

void mat_trans_single_wrapper(float x, float y, float z) {
    mat_trans_single(x, y, z);
}

void mat_trans_single4_wrapper(float x, float y, float z, float w) {
    mat_trans_single4(x, y, z, w);
}

void mat_trans_single3_wrapper(float x, float y, float z) {
    mat_trans_single3(x, y, z);
}

void mat_trans_nodiv_wrapper(float x, float y, float z, float w) {
    mat_trans_nodiv(x, y, z, w);
}

void mat_trans_single3_nodiv_wrapper(float x, float y, float z) {
    mat_trans_single3_nodiv(x, y, z);
}

void mat_trans_single3_nomod_wrapper(float x, float y, float z,
                                  float x2, float y2, float z2) {
    (void) x2;
    (void) y2;
    (void) z2;
    mat_trans_single3_nomod(x, y, z, x2, y2, z2);
}

void mat_trans_single3_nodiv_nomod_wrapper(float x, float y, float z,
                                        float x2, float y2, float z2) {
    (void) x2;
    (void) y2;
    (void) z2;
    mat_trans_single3_nodiv_nomod(x, y, z, x2, y2, z2);
}

void mat_trans_single3_nodivw_wrapper(float x, float y, float z, float w) {
    (void) w;
    mat_trans_single3_nodivw(x, y, z, w);
}

void mat_trans_single3_nodiv_div_wrapper(float x, float y, float z,
                                      float xd, float yd, float zd) {
    (void) xd;
    (void) yd;
    (void) zd;
    mat_trans_single3_nodiv_div(x, y, z, xd, yd, zd);
}

void mat_trans_normal3_wrapper(float x, float y, float z) {
    mat_trans_normal3(x, y, z);
}

void mat_trans_normal3_nomod_wrapper(float x, float y, float z,
                                  float x2, float y2, float z2) {
    (void) x2;
    (void) y2;
    (void) z2;
    mat_trans_normal3_nomod(x, y, z, x2, y2, z2);
}
