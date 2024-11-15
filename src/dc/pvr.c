#include <dc/pvr.h>

uint32_t PVR_PACK_16BIT_UV_STUB(float u, float v) {
    return PVR_PACK_16BIT_UV(u, v);
}

void pvr_set_pal_entry_stub(uint32_t idx, uint32_t value) {
    return pvr_set_pal_entry(idx, value);
}

void pvr_dr_commit_stub(const void *addr) {
    return pvr_dr_commit(addr);
}
