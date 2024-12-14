#include <arch/cache.h>

void dcache_pref_block_stub(const void *src) {
    dcache_pref_block(src);
}

void dcache_wback_sq_stub(const void *ptr) {
    dcache_wback_sq(ptr);
}

void dcache_alloc_block_stub(const void *src, uint32_t value) {
    dcache_alloc_block(src, value);
}
