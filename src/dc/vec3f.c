#include <dc/vec3f.h>

void vec3f_dot_stub(float x1, float y1, float z1,
                    float x2, float y2, float z2, float w) {
    vec3f_dot(x1, y1, z1, x2, y2, z2, w);
}

void vec3f_length_stub(float x, float y, float z, float w) {
    vec3f_length(x, y, z, w);
}

void vec3f_distance_stub(float x1, float y1, float z1,
                         float x2, float y2, float z2, float w) {
    vec3f_distance(x1, y1, z1, x2, y2, z2, w);                         
}

void vec3f_normalize_stub(float x, float y, float z) {
    vec3f_normalize(x, y, z);
}

void vec3f_sub_normalize_stub(float x1, float y1, float z1,
                              float x2, float y2, float z2,
                              float x3, float y3, float z3) {
    vec3f_sub_normalize(x1, y1, z1, x2, y2, z2, x3, y3, z3);
}

void vec3f_rotr_xy_stub(float px, float py, float pz,
                        float cx, float cy, float cz, float r) {
    vec3f_rotr_xy(px, py, pz, cx, cy, cz, r);
}

void vec3f_rotr_xz_stub(float px, float py, float pz, 
                        float cx, float cy, float cz, float r) {
    vec3f_rotr_xz(px, py, pz, cx, cy, cz, r);
}

void vec3f_rotr_yz_stub(float px, float py, float pz,
                        float cx, float cy, float cz, float r) {
    vec3f_rotr_yz(px, py, pz, cx, cy, cz, r);
}

void vec3f_rotd_xy_stub(float px, float py, float pz,
                        float cx, float cy, float cz, float r) {
    vec3f_rotd_xy(px, py, pz, cx, cy, cz, r);                        
}

void vec3f_rotd_xz_stub(float px, float py, float pz,
                        float cx, float cy, float cz, float r) {
    vec3f_rotd_xz(px, py, pz, cx, cy, cz, r);                        
}

void vec3f_rotd_yz_stub(float px, float py, float pz,
                        float cx, float cy, float cz, float r) {
    vec3f_rotd_yz(px, py, pz, cx, cy, cz, r);                        
}