#include <dc/g2bus.h>

g2_ctx_t g2_lock_stub(void) {
    return g2_lock();
}

void g2_unlock_stub(g2_ctx_t ctx) {
    return g2_unlock(ctx);
}

