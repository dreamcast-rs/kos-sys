#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use crate::prelude::*;

pub const RT_IDR0: u32                  = 0x00;
pub const RT_IDR1: u32                  = 0x01;
pub const RT_IDR2: u32                  = 0x02;
pub const RT_IDR3: u32                  = 0x03;
pub const RT_IDR4: u32                  = 0x04;
pub const RT_IDR5: u32                  = 0x05;
pub const RT_RES06: u32                 = 0x06;
pub const RT_RES07: u32                 = 0x07;
pub const RT_MAR0: u32                  = 0x08;
pub const RT_MAR1: u32                  = 0x09;
pub const RT_MAR2: u32                  = 0x0A;
pub const RT_MAR3: u32                  = 0x0B;
pub const RT_MAR4: u32                  = 0x0C;
pub const RT_MAR5: u32                  = 0x0D;
pub const RT_MAR6: u32                  = 0x0E;
pub const RT_MAR7: u32                  = 0x0F;
pub const RT_TXSTATUS0: u32             = 0x10;
pub const RT_TXSTATUS1: u32             = 0x14;
pub const RT_TXSTATUS2: u32             = 0x18;
pub const RT_TXSTATUS3: u32             = 0x1C;
pub const RT_TXADDR0: u32               = 0x20;
pub const RT_TXADDR1: u32               = 0x24;
pub const RT_TXADDR2: u32               = 0x28;
pub const RT_TXADDR3: u32               = 0x2C;
pub const RT_RXBUF: u32                 = 0x30;
pub const RT_RXEARLYCNT: u32            = 0x34;
pub const RT_RXEARLYSTATUS: u32         = 0x36;
pub const RT_CHIPCMD: u32               = 0x37;
pub const RT_RXBUFTAIL: u32             = 0x38;
pub const RT_RXBUFHEAD: u32             = 0x3A;
pub const RT_INTRMASK: u32              = 0x3C;
pub const RT_INTRSTATUS: u32            = 0x3E;
pub const RT_TXCONFIG: u32              = 0x40;
pub const RT_RXCONFIG: u32              = 0x44;
pub const RT_TIMER: u32                 = 0x48;
pub const RT_RXMISSED: u32              = 0x4C;
pub const RT_CFG9346: u32               = 0x50;
pub const RT_CONFIG0: u32               = 0x51;
pub const RT_CONFIG1: u32               = 0x52;
pub const RT_RES53: u32                 = 0x53;
pub const RT_TIMERINT: u32              = 0x54;
pub const RT_MEDIASTATUS: u32           = 0x58;
pub const RT_CONFIG3: u32               = 0x59;
pub const RT_CONFIG4: u32               = 0x5A;
pub const RT_RES5B: u32                 = 0x5B;
pub const RT_MULTIINTR: u32             = 0x5C;
pub const RT_RERID: u32                 = 0x5E;
pub const RT_RES5F: u32                 = 0x5F;
pub const RT_MII_TSAD: u32              = 0x60;
pub const RT_MII_BMCR: u32              = 0x62;
pub const RT_MII_BMSR: u32              = 0x64;
pub const RT_AS_ADVERT: u32             = 0x66;
pub const RT_AS_LPAR: u32               = 0x68;
pub const RT_AS_EXPANSION: u32          = 0x6A;

pub const RT_CONFIG5: u32               = 0xD8;

pub const RT_MII_RESET: u32             = 0x8000;
pub const RT_MII_RES4000: u32           = 0x4000;
pub const RT_MII_SPD_SET: u32           = 0x2000;
pub const RT_MII_AN_ENABLE: u32         = 0x1000;
pub const RT_MII_RES0800: u32           = 0x0800;
pub const RT_MII_RES0400: u32           = 0x0400;
pub const RT_MII_AN_START: u32          = 0x0200;
pub const RT_MII_DUPLEX: u32            = 0x0100;

pub const RT_MII_LINK: u32              = 0x0004;
pub const RT_MII_AN_CAPABLE: u32        = 0x0008;
pub const RT_MII_AN_COMPLETE: u32       = 0x0020;
pub const RT_MII_10_HALF: u32           = 0x0800;
pub const RT_MII_10_FULL: u32           = 0x1000;
pub const RT_MII_100_HALF: u32          = 0x2000;
pub const RT_MII_100_FULL: u32          = 0x4000;

pub const RT_CMD_RESET: u32             = 0x10;
pub const RT_CMD_RX_ENABLE: u32         = 0x08;
pub const RT_CMD_TX_ENABLE: u32         = 0x04;
pub const RT_CMD_RX_BUF_EMPTY: u32      = 0x01;

pub const RT_INT_PCIERR: u32            = 0x8000;
pub const RT_INT_TIMEOUT: u32           = 0x4000;
pub const RT_INT_RXFIFO_OVERFLOW: u32   = 0x0040;
pub const RT_INT_RXFIFO_UNDERRUN: u32   = 0x0020;
pub const RT_INT_LINK_CHANGE: u32       = 0x0020;
pub const RT_INT_RXBUF_OVERFLOW: u32    = 0x0010;
pub const RT_INT_TX_ERR: u32            = 0x0008;
pub const RT_INT_TX_OK: u32             = 0x0004;
pub const RT_INT_RX_ERR: u32            = 0x0002;
pub const RT_INT_RX_OK: u32             = 0x0001;

pub const RT_INT_RX_ACK: u32            = RT_INT_RXFIFO_OVERFLOW |
                                          RT_INT_RXBUF_OVERFLOW |
                                          RT_INT_RX_OK;

pub const RT_TX_CARRIER_LOST: u32       = 0x80000000;
pub const RT_TX_ABORTED: u32            = 0x40000000;
pub const RT_TX_OUT_OF_WINDOW: u32      = 0x20000000;
pub const RT_TX_STATUS_OK: u32          = 0x00008000;
pub const RT_TX_UNDERRUN: u32           = 0x00004000;
pub const RT_TX_HOST_OWNS: u32          = 0x00002000;
pub const RT_TX_SIZE_MASK: u32          = 0x00001FFF;

pub const RT_RX_MULTICAST: u32          = 0x8000;
pub const RT_RX_PAM : u32               = 0x4000;
pub const RT_RX_BROADCAST: u32          = 0x2000;
pub const RT_RX_BAD_SYMBOL: u32         = 0x0020;
pub const RT_RX_RUNT: u32               = 0x0010;
pub const RT_RX_TOO_LONG: u32           = 0x0008;
pub const RT_RX_CRC_ERR: u32            = 0x0004;
pub const RT_RX_FRAME_ALIGN: u32        = 0x0002;
pub const RT_RX_STATUS_OK: u32          = 0x0001;

#[macro_export]
macro_rules! RT_ERTH {
    ($x:expr) => {
        $x << 24
    };
}

pub const RT_RXC_MulERINT: u32          = 0x00020000;
pub const RT_RXC_RER8: u32              = 0x00010000;
#[macro_export]
macro_rules! RT_RXC_RXFTH {
    ($x:expr) => {
        $x << 13
    };
}
#[macro_export]
macro_rules! RT_RXC_RBLEN {
    ($x:expr) => {
        $x << 11
    };
}
#[macro_export]
macro_rules! RT_RXC_MXDMA {
    ($x:expr) => {
        $x << 8
    };
}

pub const RT_RXC_WRAP: u32              = 0x00000080;
pub const RT_RXC_9356SEL: u32           = 0x00000040;
pub const RT_RXC_AER: u32               = 0x00000020;
pub const RT_RXC_AR: u32                = 0x00000010;
pub const RT_RXC_AB: u32                = 0x00000008;
pub const RT_RXC_AM: u32                = 0x00000004;
pub const RT_RXC_APM: u32               = 0x00000002;
pub const RT_RXC_AAP: u32               = 0x00000001;

pub const RT_CONFIG1_LED1: u32          = 0x80;
pub const RT_CONFIG1_LED0: u32          = 0x40;
pub const RT_CONFIG1_DVRLOAD: u32       = 0x20;
pub const RT_CONFIG1_LWACT: u32         = 0x10;
pub const RT_CONFIG1_MEMMAP: u32        = 0x08;
pub const RT_CONFIG1_IOMAP: u32         = 0x04;
pub const RT_CONFIG1_VPD: u32           = 0x02;
pub const RT_CONFIG1_PMEn: u32          = 0x01;

pub const RT_CONFIG4_RxFIFIOAC: u32     = 0x80;
pub const RT_CONFIG4_AnaOff: u32        = 0x40;
pub const RT_CONFIG4_LongWF: u32        = 0x20;
pub const RT_CONFIG4_LWPME: u32         = 0x10;
pub const RT_CONFIG4_RES08: u32         = 0x08;
pub const RT_CONFIG4_LWPTN: u32         = 0x04;
pub const RT_CONFIG4_RES02: u32         = 0x02;
pub const RT_CONFIG4_PBWake: u32        = 0x01;

pub const RT_CONFIG5_RES80: u32         = 0x80;
pub const RT_CONFIG5_BWF: u32           = 0x40;
pub const RT_CONFIG5_MWF: u32           = 0x20;
pub const RT_CONFIG5_UWF: u32           = 0x10;
pub const RT_CONFIG5_FIFOAddr: u32      = 0x08;
pub const RT_CONFIG5_LDPS: u32          = 0x04;
pub const RT_CONFIG5_LANW: u32          = 0x02;
pub const RT_CONFIG5_PME_STS: u32       = 0x01;

pub type eth_rx_callback_t              = Option<extern "C" fn(pkt: *mut u8, len: c_int)>;

pub const BBA_TX_OK: c_int              = 0;
pub const BBA_TX_ERROR: c_int           = -1;
pub const BBA_TX_AGAIN: c_int           = -2;

pub const BBA_TX_NOWAIT: c_int          = 0;
pub const BBA_TX_WAIT: c_int            = 1;


extern "C" {
    pub fn bba_get_mac(arr: *mut u8);
    pub fn bba_set_rx_callback(cb: eth_rx_callback_t);
    pub fn bba_tx(pkt: *const u8, len: c_int, wait: c_int) -> c_int;
    pub fn bba_init() -> c_int;
    pub fn bba_shutdown() -> c_int;
}