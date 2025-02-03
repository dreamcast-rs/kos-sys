// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub const ASIC_EVT_PVR_RENDERDONE_VIDEO: u16    = 0x0000;
pub const ASIC_EVT_PVR_RENDERDONE_ISP: u16      = 0x0001;
pub const ASIC_EVT_PVR_RENDERDONE_TSP: u16      = 0x0002;
pub const ASIC_EVT_PVR_VBLANK_BEGIN: u16        = 0x0003;
pub const ASIC_EVT_PVR_VBLANK_END: u16          = 0x0004;
pub const ASIC_EVT_PVR_HBLANK_BEGIN: u16        = 0x0005;

pub const ASIC_EVT_PVR_YUV_DONE: u16            = 0x0006;
pub const ASIC_EVT_PVR_OPAQUEDONE: u16          = 0x0007;
pub const ASIC_EVT_PVR_OPAQUEMODDONE: u16       = 0x0008;
pub const ASIC_EVT_PVR_TRANSDONE: u16           = 0x0009;
pub const ASIC_EVT_PVR_TRANSMODDONE : u16       = 0x000a;

pub const ASIC_EVT_PVR_DMA: u16                 = 0x0013;
pub const ASIC_EVT_PVR_PTDONE: u16              = 0x0015;

pub const ASIC_EVT_PVR_ISP_OUTOFMEM: u16        = 0x0200;
pub const ASIC_EVT_PVR_STRIP_HALT: u16          = 0x0201;
pub const ASIC_EVT_PVR_PARAM_OUTOFMEM: u16      = 0x0202;
pub const ASIC_EVT_PVR_OPB_OUTOFMEM: u16        = 0x0203;
pub const ASIC_EVT_PVR_TA_INPUT_ERR: u16        = 0x0204;
pub const ASIC_EVT_PVR_TA_INPUT_OVERFLOW: u16   = 0x0205;

pub const ASIC_EVT_GD_COMMAND: u16              = 0x0100;
pub const ASIC_EVT_GD_DMA: u16                  = 0x000e;
pub const ASIC_EVT_GD_DMA_OVERRUN: u16          = 0x020d;
pub const ASIC_EVT_GD_DMA_ILLADDR: u16          = 0x020c;

pub const ASIC_EVT_MAPLE_DMA: u16               = 0x000c;
pub const ASIC_EVT_MAPLE_ERROR: u16             = 0x000d;

pub const ASIC_EVT_SPU_DMA: u16                 = 0x000f;
pub const ASIC_EVT_SPU_IRQ: u16                 = 0x0101;

pub const ASIC_EVT_G2_DMA0: u16                 = 0x000f;
pub const ASIC_EVT_G2_DMA1: u16                 = 0x0010;
pub const ASIC_EVT_G2_DMA2: u16                 = 0x0011;
pub const ASIC_EVT_G2_DMA3: u16                 = 0x0012;

pub const ASIC_EVT_EXP_8BIT: u16                = 0x0102;
pub const ASIC_EVT_EXP_PCI: u16                 = 0x0103;

pub const ASIC_ACK_A: c_uintptr_t               = 0xa05f6900;
pub const ASIC_ACK_B: c_uintptr_t               = 0xa05f6904;
pub const ASIC_ACK_C: c_uintptr_t               = 0xa05f6908;

pub const ASIC_IRQD_A: c_uintptr_t              = 0xa05f6910;
pub const ASIC_IRQD_B: c_uintptr_t              = 0xa05f6914;
pub const ASIC_IRQD_C: c_uintptr_t              = 0xa05f6918;
pub const ASIC_IRQB_A: c_uintptr_t              = 0xa05f6920;
pub const ASIC_IRQB_B: c_uintptr_t              = 0xa05f6924;
pub const ASIC_IRQB_C: c_uintptr_t              = 0xa05f6928;
pub const ASIC_IRQ9_A: c_uintptr_t              = 0xa05f6930;
pub const ASIC_IRQ9_B: c_uintptr_t              = 0xa05f6934;
pub const ASIC_IRQ9_C: c_uintptr_t              = 0xa05f6938;

pub const ASIC_IRQ9: u8                         = 0;
pub const ASIC_IRQB: u8                         = 1;
pub const ASIC_IRQD: u8                         = 2;

pub const ASIC_IRQ_MAX: u8                      = 3;
pub const ASIC_IRQ_DEFAULT: u8                  = ASIC_IRQ9;

pub type asic_evt_handler = Option<unsafe extern "C" fn(code: u32, data: *mut c_void)>;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn asic_evt_set_handler(code: u16, handler: asic_evt_handler, data: *mut c_void);
    pub fn asic_evt_request_threaded_handler(code: u16, handler: asic_evt_handler,
                                             data: *mut c_void,
                                             ack_and_mask: Option<unsafe extern "C"
                                             fn(u16)>, unmask: Option<unsafe extern "C"
                                             fn(u16)>) -> c_int;
    pub fn asic_evt_remove_handler(code: u16);
    pub fn asic_evt_disable_all();
    pub fn asic_evt_disable(code: u16, irqlevel: u8);
    pub fn asic_evt_enable(code: u16, irqlevel: u8);
    pub fn asic_init();
    pub fn asic_shutdown();
}
