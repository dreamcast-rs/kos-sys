#![allow(non_camel_case_types)]

#[repr(C)]
pub enum perf_cntr_t {
    PRFC0,
    PRFC1,
}

#[repr(C)]
pub enum perf_cntr_clock_t {
    PMCR_COUNT_CPU_CYCLES,
    PMCR_COUNT_RATIO_CYCLES,
}

#[repr(C)]
pub enum perf_cntr_event_t {
    PMCR_INIT_NO_MODE                         = 0x00,
    PMCR_OPERAND_READ_ACCESS_MODE             = 0x01,
    PMCR_OPERAND_WRITE_ACCESS_MODE            = 0x02,
    PMCR_UTLB_MISS_MODE                       = 0x03,
    PMCR_OPERAND_CACHE_READ_MISS_MODE         = 0x04,
    PMCR_OPERAND_CACHE_WRITE_MISS_MODE        = 0x05,
    PMCR_INSTRUCTION_FETCH_MODE               = 0x06,
    PMCR_INSTRUCTION_TLB_MISS_MODE            = 0x07,
    PMCR_INSTRUCTION_CACHE_MISS_MODE          = 0x08,
    PMCR_ALL_OPERAND_ACCESS_MODE              = 0x09,
    PMCR_ALL_INSTRUCTION_FETCH_MODE           = 0x0A,
    PMCR_ON_CHIP_RAM_OPERAND_ACCESS_MODE      = 0x0B,
    PMCR_ON_CHIP_IO_ACCESS_MODE               = 0x0D,
    PMCR_OPERAND_ACCESS_MODE                  = 0x0E,
    PMCR_OPERAND_CACHE_MISS_MODE              = 0x0F,
    PMCR_BRANCH_ISSUED_MODE                   = 0x10,
    PMCR_BRANCH_TAKEN_MODE                    = 0x11,
    PMCR_SUBROUTINE_ISSUED_MODE               = 0x12,
    PMCR_INSTRUCTION_ISSUED_MODE              = 0x13,
    PMCR_PARALLEL_INSTRUCTION_ISSUED_MODE     = 0x14,
    PMCR_FPU_INSTRUCTION_ISSUED_MODE          = 0x15,
    PMCR_INTERRUPT_COUNTER_MODE               = 0x16,
    PMCR_NMI_COUNTER_MODE                     = 0x17,
    PMCR_TRAPA_INSTRUCTION_COUNTER_MODE       = 0x18,
    PMCR_UBC_A_MATCH_MODE                     = 0x19,
    PMCR_UBC_B_MATCH_MODE                     = 0x1a,
    PMCR_INSTRUCTION_CACHE_FILL_MODE          = 0x21,
    PMCR_OPERAND_CACHE_FILL_MODE              = 0x22,
    PMCR_ELAPSED_TIME_MODE                    = 0x23, 
    PMCR_PIPELINE_FREEZE_BY_ICACHE_MISS_MODE  = 0x24,
    PMCR_PIPELINE_FREEZE_BY_DCACHE_MISS_MODE  = 0x25,
    PMCR_PIPELINE_FREEZE_BY_BRANCH_MODE       = 0x27,
    PMCR_PIPELINE_FREEZE_BY_CPU_REGISTER_MODE = 0x28,
    PMCR_PIPELINE_FREEZE_BY_FPU_MODE          = 0x29,
}

extern "C" {
    pub fn perf_cntr_config(counter: perf_cntr_t, event_mode: *mut perf_cntr_event_t,
                            clock_type: *mut perf_cntr_clock_t) -> bool;
    pub fn perf_cntr_start(counter: perf_cntr_t, event_mode: perf_cntr_event_t,
                           clock_type: perf_cntr_clock_t);
    pub fn perf_cntr_stop(counter: perf_cntr_t);
    pub fn perf_cntr_resume(counter: perf_cntr_t);
    pub fn perf_cntr_clear(counter: perf_cntr_t);
    pub fn perf_cntr_count(counter: perf_cntr_t) -> u64;
    pub fn perf_cntr_timer_enable();
    pub fn perf_cntr_timer_disable();
    pub fn perf_cntr_timer_enabled() -> bool;
    pub fn perf_cntr_timer_ns() -> u64;
}