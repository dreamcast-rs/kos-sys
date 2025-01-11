// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

#include <stdint.h>
#include <arch/types.h>
#include <dc/fmath_base.h>

float fipr_wrapper(float x, float y, float z, float w,
                float a, float b, float c, float d) {
    return __fipr(x, y, z, w, a, b, c, d);
}

float fipr_magnitude_sqr_wrapper(float x, float y, float z, float w) {
    return __fipr_magnitude_sqr(x, y, z, w);
}

float fsin_wrapper(float r) {
    return __fsin(r);
}

float fcos_wrapper(float r) {
    return __fcos(r);
}

float ftan_wrapper(float r) {
    return __ftan(r);
}

float fisin_wrapper(int d) {
    return __fisin(d);
}

float ficos_wrapper(int d) {
    return __ficos(d);
}

float fitan_wrapper(int d) {
    return __fitan(d);
}

float fsqrt_wrapper(float f) {
    return __fsqrt(f);
}

float frsqrt_wrapper(float f) {
    return __frsqrt(f);
}

void fsincos_wrapper(float f, float *s, float *c) {
    __fsincos(f, *s, *c);
}

void fsincosr_wrapper(float f, float *s, float *c) {
    __fsincosr(f, *s, *c);
}

uint32_t pvr_pack_bump_wrapper(float h, float t, float q) {
    uint8 hp = (uint8)(h * 255.0f);
    uint8 k1 = ~hp;
    uint8 k2 = (uint8)(hp * __fsin(t));
    uint8 k3 = (uint8)(hp * __fcos(t));
    uint8 qp = (uint8)((q / (2 * F_PI)) * 255.0f);


    return (k1 << 24) | (k2 << 16) | (k3 << 8) | qp;
}
