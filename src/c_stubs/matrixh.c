#include <dc/matrix.h>
#include <dc/vector.h>

void mat_trans_single_stub(float x, float y, float z) {
    mat_trans_single(x, y, z);
}

void mat_trans_single4_stub(float x, float y, float z, float w) {
    mat_trans_single4(x, y, z, w);
}

void mat_trans_single3_stub(float x, float y, float z) {
    mat_trans_single3(x, y, z);
}

void mat_trans_nodiv_stub(float x, float y, float z, float w) {
    mat_trans_nodiv(x, y, z, w);
}

void mat_trans_single3_nodiv_stub(float x, float y, float z) {
    mat_trans_single3_nodiv(x, y, z);
}

void mat_trans_single3_nomod_stub(float x, float y, float z,
                                  float x2, float y2, float z2) {
    mat_trans_single3_nomod(x, y, z, x2, y2, z2);                      
}

void mat_trans_single3_nodiv_nomod_stub(float x, float y, float z,
                                        float x2, float y2, float z2) {
    mat_trans_single3_nodiv_nomod(x, y, z, x2, y2, z2);                                   
}

void mat_trans_single3_nodivw_stub(float x, float y, float z, float w) {
    mat_trans_single3_nodivw(x, y, z, w);
}

void mat_trans_single3_nodiv_div_stub(float x, float y, float z,
                                      float xd, float yd, float zd) {
    mat_trans_single3_nodiv_div(x, y, z, xd, yd, zd);
}

void mat_trans_normal3_stub(float x, float y, float z) {
    mat_trans_normal3(x, y, z);
}

void mat_trans_normal3_nomod_stub(float x, float y, float z,
                                  float x2, float y2, float z2) {
    mat_trans_normal3_nomod(x, y, z, x2, y2, z2);
}