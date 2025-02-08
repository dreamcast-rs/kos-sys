// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

use super::socket::sa_family_t;

// in.h

pub type in_port_t = u16;
pub type in_addr_t = u32;

#[repr(C)]
pub struct in_addr {
    pub s_addr:         in_addr_t,
}

#[repr(C)]
pub union __s6_addr {
    pub __s6_addr8:     [u8; 16],
    pub __s6_addr16:    [u16; 8],
    pub __s6_addr32:    [u32; 4],
    pub __s6_addr64:    [u64; 2],
}

#[repr(C)]
pub struct in6_addr {
    pub __s6_addr:      __s6_addr,
}

#[repr(C)]
pub struct sockaddr_in {
    sin_family:         sa_family_t,
    sin_port:           in_port_t,
    sin_addr:           in_addr,
    sin_zero:           [c_uchar; 8],
}

#[repr(C)]
pub struct sockaddr_in6 {
    sin6_family:        sa_family_t,
    sin6_port:          in_port_t,
    sin6_flowinfo:      u32,
    sin6_addr:          in6_addr,
    sin6_scope_id:      u32,
}

pub const INADDR_ANY: in_addr               = in_addr { s_addr: 0x00000000 };
pub const INADDR_BROADCAST: in_addr         = in_addr { s_addr: 0xFFFFFFFF };
pub const INADDR_NONE: in_addr              = in_addr { s_addr: 0xFFFFFFFF };

pub const IN6ADDR_ANY_INIT: in6_addr        = in6_addr {
    __s6_addr: __s6_addr {
        __s6_addr8: [0u8; 16],
    },
};

pub const IN6ADDR_LOOPBACK_INIT: in6_addr   = in6_addr {
    __s6_addr: __s6_addr {
        __s6_addr8: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    },
};

unsafe extern "C" {
    pub static in6addr_any: in6_addr;
    pub static in6addr_loopback: in6_addr;
}

pub const INET_ADDRSTRLEN: usize            = 16;
pub const INET6_ADDRSTRLEN: usize           = 46;

pub const IPPROTO_IP: c_int                 = 0;
pub const IPPROTO_ICMP: c_int               = 1;
pub const IPPROTO_TCP: c_int                = 6;
pub const IPPROTO_UDP: c_int                = 17;
pub const IPPROTO_IPV6: c_int               = 41;
pub const IPPROTO_UDPLITE: c_int            = 136;

pub const IP_TTL: u8                        = 24;

pub const IPV6_JOIN_GROUP: u8               = 17;
pub const IPV6_LEAVE_GROUP: u8              = 18;
pub const IPV6_MULTICAST_HOPS: u8           = 19;
pub const IPV6_MULTICAST_IF: u8             = 20;
pub const IPV6_MULTICAST_LOOP: u8           = 21;
pub const IPV6_UNICAST_HOPS: u8             = 22;
pub const IPV6_V6ONLY: u8                   = 23;

#[inline]
pub const fn IN6_IS_ADDR_UNSPECIFIED(a: *const in6_addr) -> bool {
    unsafe {
        (*a).__s6_addr.__s6_addr32[0] == 0 &&
        (*a).__s6_addr.__s6_addr32[1] == 0 &&
        (*a).__s6_addr.__s6_addr32[2] == 0 &&
        (*a).__s6_addr.__s6_addr32[3] == 0
    }
}

#[inline]
pub const fn IN6_IS_ADDR_LOOPBACK(a: *const in6_addr) -> bool {
    unsafe {
        (*a).__s6_addr.__s6_addr32[0] == 0 &&
        (*a).__s6_addr.__s6_addr32[1] == 0 &&
        (*a).__s6_addr.__s6_addr32[2] == 0 &&
        (*a).__s6_addr.__s6_addr16[6] == 0 &&
        (*a).__s6_addr.__s6_addr8[14] == 0 &&
        (*a).__s6_addr.__s6_addr8[15] == 1
    }
}

#[inline]
pub const fn IN6_IS_ADDR_V4MAPPED(a: *const in6_addr) -> bool {
    unsafe {
        (*a).__s6_addr.__s6_addr32[0] == 0 &&
        (*a).__s6_addr.__s6_addr32[1] == 0 &&
        (*a).__s6_addr.__s6_addr16[4] == 0 &&
        (*a).__s6_addr.__s6_addr16[5] == 0xFFFF
    }
}

#[inline]
pub const fn IN6_IS_ADDR_V4COMPAT(a: *const in6_addr) -> bool {
    unsafe {
        (*a).__s6_addr.__s6_addr32[0] == 0 &&
        (*a).__s6_addr.__s6_addr32[1] == 0 &&
        (*a).__s6_addr.__s6_addr32[2] == 0 &&
        (*a).__s6_addr.__s6_addr32[3] != 0 &&
        (*a).__s6_addr.__s6_addr8[15] != 1
    }
}

#[inline]
pub const fn IN6_IS_ADDR_LINKLOCAL(a: *const in6_addr) -> bool {
    unsafe {
        ((*a).__s6_addr.__s6_addr8[0] == 0xFE) &&
        (((*a).__s6_addr.__s6_addr8[1] & 0xC0) == 0x80)
    }
}

#[inline]
pub const fn IN6_IS_ADDR_SITELOCAL(a: *const in6_addr) -> bool {
    unsafe {
        ((*a).__s6_addr.__s6_addr8[0] == 0xFE) &&
        (((*a).__s6_addr.__s6_addr8[1] & 0xC0) == 0xC0)
    }
}

#[inline]
pub const fn IN6_IS_ADDR_MULTICAST(a: *const in6_addr) -> bool {
    unsafe {
        (*a).__s6_addr.__s6_addr8[0] == 0xFF
    }
}

#[inline]
pub const fn IN6_IS_ADDR_MC_NODELOCAL(a: *const in6_addr) -> bool {
    unsafe {
        IN6_IS_ADDR_MULTICAST(a) &&
        (((*a).__s6_addr.__s6_addr8[1] & 0x0F) == 0x01)
    }
}

#[inline]
pub const fn IN6_IS_ADDR_MC_LINKLOCAL(a: *const in6_addr) -> bool {
    unsafe {
        IN6_IS_ADDR_MULTICAST(a) &&
        (((*a).__s6_addr.__s6_addr8[1] & 0x0F) == 0x02)
    }
}

#[inline]
pub const fn IN6_IS_ADDR_MC_SITELOCAL(a: *const in6_addr) -> bool {
    unsafe {
        IN6_IS_ADDR_MULTICAST(a) &&
        (((*a).__s6_addr.__s6_addr8[1] & 0x0F) == 0x05)
    }
}

#[inline]
pub const fn IN6_IS_ADDR_MC_ORGLOCAL(a: *const in6_addr) -> bool {
    unsafe {
        IN6_IS_ADDR_MULTICAST(a) &&
        (((*a).__s6_addr.__s6_addr8[1] & 0x0F) == 0x08)
    }
}

#[inline]
pub const fn IN6_IS_ADDR_MC_GLOBAL(a: *const in6_addr) -> bool {
    unsafe {
        IN6_IS_ADDR_MULTICAST(a) &&
        (((*a).__s6_addr.__s6_addr8[1] & 0x0F) == 0x0E)
    }
}

// tcp.h

pub const TCP_NODELAY: c_int                = 1;

// udp.h

pub const UDP_NOCHECKSUM: c_int             = 25;

// udplite.h

pub const UDPLITE_SEND_CSCOV: c_int         = 26;
pub const UDPLITE_RECV_CSCOV: c_int         = 27;
