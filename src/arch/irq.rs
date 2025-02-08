// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub const REG_BYTE_CNT: usize           = 256;

#[repr(C, align(32))]
pub struct irq_context_t {
    pc:     u32,
    pr:     u32,
    gbr:    u32,
    vbr:    u32,
    mach:   u32,
    macl:   u32,
    sr:     u32,
    fpul:   u32,
    fr:     [u32; 16],
    frbank: [u32; 16],
    r:      [u32; 16],
    fpscr:  u32
}

#[inline]
pub const fn CONTEXT_PC(cxt: irq_context_t) -> u32 {
    cxt.pc
}

#[inline]
pub const fn CONTEXT_FP(cxt: irq_context_t) -> u32 {
    cxt.r[14]
}

#[inline]
pub const fn CONTEXT_SP(cxt: irq_context_t) -> u32 {
    cxt.r[15]
}

#[inline]
pub const fn CONTEXT_RET(cxt: irq_context_t) -> u32 {
    cxt.r[0]
}

#[repr(C)]
pub enum irq_t {
    EXC_RESET_POWERON       = 0x0000,
    EXC_RESET_MANUAL        = 0x0020,
//  EXC_RESET_UDI           = 0x0000,
    EXC_ITLB_MULTIPLE       = 0x0140,
//  EXC_DTLB_MULTIPLE       = 0x0140,

    EXC_USER_BREAK_PRE      = 0x01e0,
    EXC_INSTR_ADDRESS       = 0x00e0,
    EXC_ITLB_MISS           = 0x0040,
    EXC_ITLB_PV             = 0x00a0,
    EXC_ILLEGAL_INSTR       = 0x0180,
    EXC_SLOT_ILLEGAL_INSTR  = 0x01a0,
    EXC_GENERAL_FPU         = 0x0800,
    EXC_SLOT_FPU            = 0x0820,
//  EXC_DATA_ADDRESS_READ   = 0x00e0,
    EXC_DATA_ADDRESS_WRITE  = 0x0100,
//  EXC_DTLB_MISS_READ      = 0x0040,
    EXC_DTLB_MISS_WRITE     = 0x0060,
//  EXC_DTLB_PV_READ        = 0x00a0,
    EXC_DTLB_PV_WRITE       = 0x00c0,
    EXC_FPU                 = 0x0120,
    EXC_INITIAL_PAGE_WRITE  = 0x0080,

    EXC_TRAPA               = 0x0160,
//  EXC_USER_BREAK_POST     = 0x01e0,

    EXC_NMI                 = 0x01c0,
    EXC_IRQ0                = 0x0200,
    EXC_IRQ1                = 0x0220,
    EXC_IRQ2                = 0x0240,
    EXC_IRQ3                = 0x0260,
    EXC_IRQ4                = 0x0280,
    EXC_IRQ5                = 0x02a0,
    EXC_IRQ6                = 0x02c0,
    EXC_IRQ7                = 0x02e0,
    EXC_IRQ8                = 0x0300,
    EXC_IRQ9                = 0x0320,
    EXC_IRQA                = 0x0340,
    EXC_IRQB                = 0x0360,
    EXC_IRQC                = 0x0380,
    EXC_IRQD                = 0x03a0,
    EXC_IRQE                = 0x03c0,
    EXC_TMU0_TUNI0          = 0x0400,
    EXC_TMU1_TUNI1          = 0x0420,
    EXC_TMU2_TUNI2          = 0x0440,
    EXC_TMU2_TICPI2         = 0x0460,
    EXC_RTC_ATI             = 0x0480,
    EXC_RTC_PRI             = 0x04a0,
    EXC_RTC_CUI             = 0x04c0,
    EXC_SCI_ERI             = 0x04e0,
    EXC_SCI_RXI             = 0x0500,
    EXC_SCI_TXI             = 0x0520,
    EXC_SCI_TEI             = 0x0540,
    EXC_WDT_ITI             = 0x0560,
    EXC_REF_RCMI            = 0x0580,
    EXC_REF_ROVI            = 0x05a0,
    EXC_UDI                 = 0x0600,
    EXC_GPIO_GPIOI          = 0x0620,
    EXC_DMAC_DMTE0          = 0x0640,
    EXC_DMAC_DMTE1          = 0x0660,
    EXC_DMAC_DMTE2          = 0x0680,
    EXC_DMAC_DMTE3          = 0x06a0,
    EXC_DMA_DMAE            = 0x06c0,
    EXC_SCIF_ERI            = 0x0700,
    EXC_SCIF_RXI            = 0x0720,
    EXC_SCIF_BRI            = 0x0740,
    EXC_SCIF_TXI            = 0x0760,

    EXC_DOUBLE_FAULT        = 0x0780,

    EXC_UNHANDLED_EXC       = 0x07e0,
}

pub const EXC_RESET_UDI: irq_t          = irq_t::EXC_RESET_POWERON;
pub const EXC_DTLB_MULTIPLE: irq_t      = irq_t::EXC_ITLB_MULTIPLE;
pub const EXC_USER_BREAK_POST: irq_t    = irq_t::EXC_USER_BREAK_PRE;
pub const EXC_DATA_ADDRESS_READ: irq_t  = irq_t::EXC_INSTR_ADDRESS;
pub const EXC_DTLB_MISS_READ: irq_t     = irq_t::EXC_ITLB_MISS;
pub const EXC_DTLB_PV_READ: irq_t       = irq_t::EXC_ITLB_PV;

pub const EXC_OFFSET_000: u32           = 0;
pub const EXC_OFFSET_100: u32           = 1;
pub const EXC_OFFSET_400: u32           = 2;
pub const EXC_OFFSET_600: u32           = 3;

pub const TIMER_IRQ: irq_t     = irq_t::EXC_TMU0_TUNI0;

pub type irq_mask_t = u32;

pub type irq_handler = Option<unsafe extern "C" fn(code: irq_t,
                                                   context: *mut irq_context_t,
                                                   data: *mut c_void)>;

#[repr(C)]
pub struct irq_cb_t {
    hdl:    irq_handler,
    data:   *mut c_void,
}

// FIXME: Unimplemented bindings for the following:
/*
    static inline void __irq_scoped_cleanup(int *state)
    #define ___irq_disable_scoped(l)
    #define __irq_disable_scoped(l)
    #define irq_disable_scoped()
*/

pub const IRQ_PRIO_MAX: c_uint          = 15;
pub const IRQ_PRIO_MIN: c_uint          = 1;
pub const IRQ_PRIO_MASKED: c_uint       = 0;

#[repr(C)]
pub enum irq_src_t {
    IRQ_SRC_RTC,
    IRQ_SRC_TMU2,
    IRQ_SRC_TMU1,
    IRQ_SRC_TMU0,
    _IRQ_SRC_RESV,
    IRQ_SRC_SCI1,
    IRQ_SRC_REF,
    IRQ_SRC_WDT,
    IRQ_SRC_HUDI,
    IRQ_SRC_SCIF,
    IRQ_SRC_DMAC,
    IRQ_SRC_GPIO,
    IRQ_SRC_IRL3,
    IRQ_SRC_IRL2,
    IRQ_SRC_IRL1,
    IRQ_SRC_IRL0,
}

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn irq_set_context(regbank: *mut irq_context_t);
    pub fn irq_get_context() -> *mut irq_context_t;
    pub fn irq_create_context(
        context: *mut irq_context_t,
        stack_pointer: u32,
        routine: u32,
        args: *const u32,
        usermode: bool,
    );
    pub fn irq_inside_int() -> c_int;
    pub fn irq_disable() -> irq_mask_t;
    pub fn irq_enable();
    pub fn irq_restore(v: irq_mask_t);
    pub fn irq_force_return();
    pub fn irq_set_handler(code: irq_t, hnd: irq_handler, data: *mut c_void) -> c_int;
    pub fn irq_get_handler(code: irq_t) -> irq_cb_t;
    pub fn irq_set_global_handler(handler: irq_handler, data: *mut c_void) -> c_int;
    pub fn irq_get_global_handler() -> irq_cb_t;
    pub fn irq_init() -> c_int;
    pub fn irq_shutdown();
    pub fn irq_set_priority(src: irq_src_t, prio: c_uint);
    pub fn irq_get_priority(src: irq_src_t) -> c_uint;
}
