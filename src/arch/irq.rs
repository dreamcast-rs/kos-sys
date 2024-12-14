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

pub const EXC_RESET_POWERON: irq_t      = 0x0000;
pub const EXC_RESET_MANUAL: irq_t       = 0x0020;
pub const EXC_RESET_UDI: irq_t          = 0x0000;
pub const EXC_ITLB_MULTIPLE: irq_t      = 0x0140;
pub const EXC_DTLB_MULTIPLE: irq_t      = 0x0140;

pub const EXC_USER_BREAK_PRE: irq_t     = 0x01e0;
pub const EXC_INSTR_ADDRESS: irq_t      = 0x00e0;
pub const EXC_ITLB_MISS: irq_t          = 0x0040;
pub const EXC_ITLB_PV: irq_t            = 0x00a0;
pub const EXC_ILLEGAL_INSTR: irq_t      = 0x0180;
pub const EXC_SLOT_ILLEGAL_INSTR: irq_t = 0x01a0;
pub const EXC_GENERAL_FPU: irq_t        = 0x0800;
pub const EXC_SLOT_FPU: irq_t           = 0x0820;
pub const EXC_DATA_ADDRESS_READ: irq_t  = 0x00e0;
pub const EXC_DATA_ADDRESS_WRITE: irq_t = 0x0100;
pub const EXC_DTLB_MISS_READ: irq_t     = 0x0040;
pub const EXC_DTLB_MISS_WRITE: irq_t    = 0x0060;
pub const EXC_DTLB_PV_READ: irq_t       = 0x00a0;
pub const EXC_DTLB_PV_WRITE: irq_t      = 0x00c0;
pub const EXC_FPU: irq_t                = 0x0120;
pub const EXC_INITIAL_PAGE_WRITE: irq_t = 0x0080;

pub const EXC_TRAPA: irq_t              = 0x0160;
pub const EXC_USER_BREAK_POST: irq_t    = 0x01e0;

pub const EXC_NMI: irq_t                = 0x01c0;
pub const EXC_IRQ0: irq_t               = 0x0200;
pub const EXC_IRQ1: irq_t               = 0x0220;
pub const EXC_IRQ2: irq_t               = 0x0240;
pub const EXC_IRQ3: irq_t               = 0x0260;
pub const EXC_IRQ4: irq_t               = 0x0280;
pub const EXC_IRQ5: irq_t               = 0x02a0;
pub const EXC_IRQ6: irq_t               = 0x02c0;
pub const EXC_IRQ7: irq_t               = 0x02e0;
pub const EXC_IRQ8: irq_t               = 0x0300;
pub const EXC_IRQ9: irq_t               = 0x0320;
pub const EXC_IRQA: irq_t               = 0x0340;
pub const EXC_IRQB: irq_t               = 0x0360;
pub const EXC_IRQC: irq_t               = 0x0380;
pub const EXC_IRQD: irq_t               = 0x03a0;
pub const EXC_IRQE: irq_t               = 0x03c0;
pub const EXC_TMU0_TUNI0: irq_t         = 0x0400;
pub const EXC_TMU1_TUNI1: irq_t         = 0x0420;
pub const EXC_TMU2_TUNI2: irq_t         = 0x0440;
pub const EXC_TMU2_TICPI2: irq_t        = 0x0460;
pub const EXC_RTC_ATI: irq_t            = 0x0480;
pub const EXC_RTC_PRI: irq_t            = 0x04a0;
pub const EXC_RTC_CUI: irq_t            = 0x04c0;
pub const EXC_SCI_ERI: irq_t            = 0x04e0;
pub const EXC_SCI_RXI: irq_t            = 0x0500;
pub const EXC_SCI_TXI: irq_t            = 0x0520;
pub const EXC_SCI_TEI: irq_t            = 0x0540;
pub const EXC_WDT_ITI: irq_t            = 0x0560;
pub const EXC_REF_RCMI: irq_t           = 0x0580;
pub const EXC_REF_ROVI: irq_t           = 0x05a0;
pub const EXC_UDI: irq_t                = 0x0600;
pub const EXC_GPIO_GPIOI: irq_t         = 0x0620;
pub const EXC_DMAC_DMTE0: irq_t         = 0x0640;
pub const EXC_DMAC_DMTE1: irq_t         = 0x0660;
pub const EXC_DMAC_DMTE2: irq_t         = 0x0680;
pub const EXC_DMAC_DMTE3: irq_t         = 0x06a0;
pub const EXC_DMA_DMAE: irq_t           = 0x06c0;
pub const EXC_SCIF_ERI: irq_t           = 0x0700;
pub const EXC_SCIF_RXI: irq_t           = 0x0720;
pub const EXC_SCIF_BRI: irq_t           = 0x0740;
pub const EXC_SCIF_TXI: irq_t           = 0x0760;

pub const EXC_DOUBLE_FAULT: irq_t       = 0x0ff0;

pub const EXC_UNHANDLED_EXC: irq_t      = 0x0fe0;

pub const EXC_OFFSET_000: u32           = 0;
pub const EXC_OFFSET_100: u32           = 1;
pub const EXC_OFFSET_400: u32           = 2;
pub const EXC_OFFSET_600: u32           = 3;

pub const TIMER_IRQ: irq_t              = EXC_TMU0_TUNI0;

pub type irq_t = u32;

pub type irq_handler = Option<unsafe extern "C" fn(source: irq_t,
                                                   context: *mut irq_context_t,
                                                   data: *mut c_void)>;

extern "C" {
    pub fn irq_inside_int() -> c_int;
    pub fn irq_force_return();
    pub fn irq_set_handler(source: irq_t, hnd: irq_handler, data: *mut c_void) -> c_int;
    pub fn irq_get_handler(source: irq_t) -> irq_handler;
    pub fn trapa_set_handler(code: irq_t, hnd: irq_handler, data: *mut c_void) -> c_int;
    pub fn irq_set_global_handler(hnd: irq_handler, data: *mut c_void) -> c_int;
    pub fn irq_get_global_handler() -> irq_handler;
    pub fn irq_set_context(regbank: *mut irq_context_t);
    pub fn irq_get_context() -> *mut irq_context_t;
    pub fn irq_create_context(context: *mut irq_context_t, stack_pointer: u32,
                              routine: u32, args: *mut u32, usermode: c_int);
    pub fn irq_disable() -> c_int;
    pub fn irq_enable();
    pub fn irq_restore(v: c_int);
    pub fn irq_init() -> c_int;
    pub fn irq_shutdown();
}

