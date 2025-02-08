// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

#include <dc/vec3f.h>

float vec_fipr_wrapper(vec3f_t vec) {
    return vec_fipr(vec);
}

float vec_dot_wrapper(vec3f_t vec1, vec3f_t vec2) {
    return vec_dot(vec1, vec2);
}

float vec_length_wrapper(vec3f_t vec) {
    return vec_length(vec);
}

float vec_distance_wrapper(vec3f_t vec1, vec3f_t vec2) {
    return vec_distance(vec1, vec2);
}

vec3f_t vec_normalize_wrapper(vec3f_t vec) {
    return vec_normalize(vec);
}

vec3f_t vec_sub_normalize_wrapper(vec3f_t vec1, vec3f_t vec2) {
    return vec_sub_normalize(vec1, vec2);
}

vec3f_t vec_rotr_xy_wrapper(vec3f_t vec, vec3f_t origin, float angle) {
    return vec_rotr_xy(vec, origin, angle);
}

vec3f_t vec_rotr_xz_wrapper(vec3f_t vec, vec3f_t origin, float angle) {
    return vec_rotr_xz(vec, origin, angle);
}

vec3f_t vec_rotr_yz_wrapper(vec3f_t vec, vec3f_t origin, float angle)  {
    return vec_rotr_yz(vec, origin, angle);
}

vec3f_t vec_rotd_xy_wrapper(vec3f_t vec, vec3f_t origin, float angle) {
    return vec_rotd_xy(vec, origin, angle);
}

vec3f_t vec_rotd_xz_wrapper(vec3f_t vec, vec3f_t origin, float angle) {
    return vec_rotd_xz(vec, origin, angle);
}

vec3f_t vec_rotd_yz_wrapper(vec3f_t vec, vec3f_t origin, float angle) {
    return vec_rotd_yz(vec, origin, angle);
}

void vec3f_dot_wrapper(float x1, float y1, float z1,
                       float x2, float y2, float z2, float w) {
    (void) w;
    vec3f_dot(x1, y1, z1, x2, y2, z2, w);
}

void vec3f_length_wrapper(float x, float y, float z, float w) {
    (void) w;
    vec3f_length(x, y, z, w);
}

void vec3f_distance_wrapper(float x1, float y1, float z1,
                            float x2, float y2, float z2, float w) {
    (void) w;
    vec3f_distance(x1, y1, z1, x2, y2, z2, w);
}

void vec3f_normalize_wrapper(float x, float y, float z) {
    vec3f_normalize(x, y, z);
}

void vec3f_sub_normalize_wrapper(float x1, float y1, float z1,
                                 float x2, float y2, float z2,
                                 float x3, float y3, float z3) {
    (void) x3;
    (void) y3;
    (void) z3;
    vec3f_sub_normalize(x1, y1, z1, x2, y2, z2, x3, y3, z3);
}

void vec3f_rotr_xy_wrapper(float px, float py, float pz,
                           float cx, float cy, float cz, float r) {
    vec3f_rotr_xy(px, py, pz, cx, cy, cz, r);
}

void vec3f_rotr_xz_wrapper(float px, float py, float pz,
                           float cx, float cy, float cz, float r) {
    vec3f_rotr_xz(px, py, pz, cx, cy, cz, r);
}

void vec3f_rotr_yz_wrapper(float px, float py, float pz,
                           float cx, float cy, float cz, float r) {
    vec3f_rotr_yz(px, py, pz, cx, cy, cz, r);
}

void vec3f_rotd_xy_wrapper(float px, float py, float pz,
                           float cx, float cy, float cz, float r) {
    vec3f_rotd_xy(px, py, pz, cx, cy, cz, r);
}

void vec3f_rotd_xz_wrapper(float px, float py, float pz,
                           float cx, float cy, float cz, float r) {
    vec3f_rotd_xz(px, py, pz, cx, cy, cz, r);
}

void vec3f_rotd_yz_wrapper(float px, float py, float pz,
                           float cx, float cy, float cz, float r) {
    vec3f_rotd_yz(px, py, pz, cx, cy, cz, r);
}
