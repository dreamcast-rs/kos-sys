// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;
use super::netinet::in6_addr;

// "self" parameters renamed to "netif"
#[repr(C)]
pub struct netif_t {
    pub le_next:                *mut netif_t,
    pub le_prev:                *mut *mut netif_t,
    pub name:                   *const c_char,
    pub descr:                  *const c_char,
    pub index:                  c_int,
    pub dev_id:                 u32,
    pub flags:                  u32,
    pub mac_addr:               [u8; 6],
    pub ip_addr:                [u8; 4],
    pub netmask:                [u8; 4],
    pub gateway:                [u8; 4],
    pub broadcast:              [u8; 4],
    pub dns:                    [u8; 4],
    pub mtu:                    c_int,
    pub ip6_lladdr:             in6_addr,
    pub ip6_addrs:              *mut in6_addr,
    pub ip6_addr_count:         c_int,
    pub ip6_gateway:            in6_addr,
    pub mtu6:                   u32,
    pub hop_limit:              c_int,
    pub if_detect:              Option<unsafe extern "C" fn(netif: *mut netif_t)
                                                            -> c_int>,
    pub if_init:                Option<unsafe extern "C" fn(netif: *mut netif_t)
                                                            -> c_int>,
    pub if_shutdown:            Option<unsafe extern "C" fn(netif: *mut netif_t)
                                                            -> c_int>,
    pub if_start:               Option<unsafe extern "C" fn(netif: *mut netif_t)
                                                            -> c_int>,
    pub if_stop:                Option<unsafe extern "C" fn(netif: *mut netif_t)
                                                            -> c_int>,
    pub if_tx:                  Option<unsafe extern "C" fn(netif: *mut netif_t,
                                                            data: *const u8, len: c_int,
                                                            blocking: c_int) -> c_int>,
    pub if_tx_commit:           Option<unsafe extern "C" fn(netif: *mut netif_t)
                                                            -> c_int>,
    pub if_rx_poll:             Option<unsafe extern "C" fn(netif: *mut netif_t)
                                                            -> c_int>,
    pub if_set_flags:           Option<unsafe extern "C" fn(netif: *mut netif_t,
                                                            flags_and: u32, flags_or: u32)
                                                            -> c_int>,
    pub if_set_mc:              Option<unsafe extern "C" fn(netif: *mut netif_t,
                                                            list: *const u8, count: c_int)
                                                            -> c_int>,
}

pub const NETIF_NO_FLAGS: u32                   = 0x00000000;
pub const NETIF_REGISTERED: u32                 = 0x00000001;
pub const NETIF_DETECTED: u32                   = 0x00000002;
pub const NETIF_INITIALIZED: u32                = 0x00000004;
pub const NETIF_RUNNING: u32                    = 0x00000008;
pub const NETIF_PROMISC: u32                    = 0x00010000;
pub const NETIF_NEEDSPOLL: u32                  = 0x01000000;
pub const NETIF_NOETH: u32                      = 0x10000000;

pub const NETIF_TX_OK: c_int                    = 0;
pub const NETIF_TX_ERROR: c_int                 = -1;
pub const NETIF_TX_AGAIN: c_int                 = -2;

pub const NETIF_NOBLOCK: c_int                  = 0;
pub const NETIF_BLOCK: c_int                    = 1;

#[repr(C)]
pub struct netif_list {
    lh_first:               *mut netif_t,
}

#[repr(C, packed)]
pub struct ip_hdr_t {
    pub version_ihl:            u8,
    pub tos:                    u8,
    pub length:                 u16,
    pub packet_id:              u16,
    pub flags_frag_offs:        u16,
    pub ttl:                    u8,
    pub protocol:               u8,
    pub checksum:               u16,
    pub src:                    u32,
    pub dest:                   u32,
}

#[repr(C, packed)]
pub struct ipv6_hdr_t {
    pub version_lclass:         u8,
    pub hclass_lflow:           u8,
    pub lclass:                 u16,
    pub length:                 u16,
    pub next_header:            u8,
    pub hop_limit:              u8,
    pub src_addr:               in6_addr,
    pub dst_addr:               in6_addr,
}

pub type net_input_func = Option<unsafe extern "C" fn(nif: *mut netif_t, pkt: *const u8,
                                                      len: c_int) -> c_int>;

pub type net_echo_cb = Option<unsafe extern "C" fn(ip: *const u8, seq: u16, delta_us: u64,
                                                   ttl: u8, data: *const u8,
                                                   len: c_size_t)>;

pub const ICMP_PROTOCOL_UNREACHABLE: u8         = 2;
pub const ICMP_PORT_UNREACHABLE: u8             = 3;

pub const ICMP_REASSEMBLY_TIME_EXCEEDED: u8     = 1;

#[repr(C)]
pub struct net_ipv4_stats_t {
    pub pkt_sent:               u32,
    pub pkt_send_failed:        u32,
    pub pkt_recv:               u32,
    pub pkt_recv_bad_size:      u32,
    pub pkt_recv_bad_chksum:    u32,
    pub pkt_recv_no_sock:       u32,
}

pub type net6_echo_cb = Option<unsafe extern "C" fn(ip: *const in6_addr, seq: u16,
                                                    delta_us: u64, hlim: u8,
                                                    data: *const u8, len: c_size_t)>;

pub const ICMP6_DEST_UNREACH_NO_ROUTE: u8       = 0;
pub const ICMP6_DEST_UNREACH_PROHIBITED: u8     = 1;
pub const ICMP6_DEST_UNREACH_BEYOND_SCOPE: u8   = 2;
pub const ICMP6_DEST_UNREACH_ADDR_UNREACH: u8   = 3;
pub const ICMP6_DEST_UNREACH_PORT_UNREACH: u8   = 4;
pub const ICMP6_DEST_UNREACH_FAIL_EGRESS: u8    = 5;
pub const ICMP6_DEST_UNREACH_BAD_ROUTE: u8      = 6;

pub const ICMP6_TIME_EXCEEDED_HOPS_EXC: u8      = 0;
pub const ICMP6_TIME_EXCEEDED_FRAGMENT: u8      = 1;

pub const ICMP6_PARAM_PROB_BAD_HEADER: u8       = 0;
pub const ICMP6_PARAM_PROB_UNK_HEADER: u8       = 1;
pub const ICMP6_PARAM_PROB_UNK_OPTION: u8       = 2;

#[repr(C)]
pub struct net_ipv6_stats_t {
    pub pkt_sent:               u32,
    pub pkt_send_failed:        u32,
    pub pkt_recv:               u32,
    pub pkt_recv_bad_size:      u32,
    pub pkt_recv_bad_proto:     u32,
    pub pkt_recv_bad_ext:       u32,
}

#[repr(C)]
pub struct net_udp_stats_t {
    pub pkt_sent:               u32,
    pub pkt_send_failed:        u32,
    pub pkt_recv:               u32,
    pub pkt_recv_bad_size:      u32,
    pub pkt_recv_bad_chksum:    u32,
    pub pkt_recv_no_sock:       u32,
}

#[link(name = "kallisti")]
unsafe extern "C" {
    pub static mut net_input_target: net_input_func;
    pub static mut net_icmp_echo_cb: net_echo_cb;
    pub static mut net_icmp6_echo_cb: net6_echo_cb;
    pub static mut net_if_list: netif_list;
    pub static mut net_default_dev: *mut netif_t;
    pub fn net_arp_init() -> c_int;
    pub fn net_arp_shutdown();
    pub fn net_arp_insert(
        nif: *mut netif_t,
        mac: *const u8,
        ip: *const u8,
        timestamp: u64,
    ) -> c_int;
    pub fn net_arp_lookup(
        nif: *mut netif_t,
        ip_in: *const u8,
        mac_out: *mut u8,
        pkt: *const ip_hdr_t,
        data: *const u8,
        data_size: c_int,
    ) -> c_int;
    pub fn net_arp_revlookup(nif: *mut netif_t, ip_out: *mut u8, mac_in: *const u8) -> c_int;
    pub fn net_arp_input(nif: *mut netif_t, pkt: *const u8, len: c_int) -> c_int;
    pub fn net_arp_query(nif: *mut netif_t, ip: *const u8) -> c_int;
    pub fn net_input(device: *mut netif_t, data: *const u8, len: c_int) -> c_int;
    pub fn net_input_set_target(t: net_input_func) -> net_input_func;
    pub fn net_icmp_send_echo(
        net: *mut netif_t,
        ipaddr: *const u8,
        ident: u16,
        seq: u16,
        data: *const u8,
        size: c_size_t,
    ) -> c_int;
    pub fn net_icmp_send_dest_unreach(net: *mut netif_t, code: u8, msg: *const u8) -> c_int;
    pub fn net_icmp_send_time_exceeded(net: *mut netif_t, code: u8, msg: *const u8) -> c_int;
    pub fn net_ipv4_get_stats() -> net_ipv4_stats_t;
    pub fn net_ipv4_address(addr: *const u8) -> u32;
    pub fn net_ipv4_parse_address(addr: u32, out: *mut u8);
    pub fn net_icmp6_send_echo(
        net: *mut netif_t,
        dst: *const in6_addr,
        ident: u16,
        seq: u16,
        data: *const u8,
        size: c_size_t,
    ) -> c_int;
    pub fn net_icmp6_send_nsol(
        net: *mut netif_t,
        dst: *const in6_addr,
        target: *const in6_addr,
        dupdet: c_int,
    ) -> c_int;
    pub fn net_icmp6_send_nadv(
        net: *mut netif_t,
        dst: *const in6_addr,
        target: *const in6_addr,
        sol: c_int,
    ) -> c_int;
    pub fn net_icmp6_send_rsol(net: *mut netif_t) -> c_int;
    pub fn net_icmp6_send_dest_unreach(
        net: *mut netif_t,
        code: u8,
        ppkt: *const u8,
        psz: c_size_t,
    ) -> c_int;
    pub fn net_icmp6_send_time_exceeded(
        net: *mut netif_t,
        code: u8,
        ppkt: *const u8,
        psz: c_size_t,
    ) -> c_int;
    pub fn net_icmp6_send_param_prob(
        net: *mut netif_t,
        code: u8,
        ptr: u32,
        ppkt: *const u8,
        psz: c_size_t,
    ) -> c_int;
    pub fn net_ipv6_get_stats() -> net_ipv6_stats_t;
    pub fn net_ndp_init() -> c_int;
    pub fn net_ndp_shutdown();
    pub fn net_ndp_gc();
    pub fn net_ndp_insert(
        nif: *mut netif_t,
        mac: *const u8,
        ip: *const in6_addr,
        unsol: c_int,
    ) -> c_int;
    pub fn net_ndp_lookup(
        net: *mut netif_t,
        ip: *const in6_addr,
        mac_out: *mut u8,
        pkt: *const ipv6_hdr_t,
        data: *const u8,
        data_size: c_int,
    ) -> c_int;
    pub fn net_udp_get_stats() -> net_udp_stats_t;
    pub fn net_udp_init() -> c_int;
    pub fn net_udp_shutdown();
    pub fn net_tcp_init() -> c_int;
    pub fn net_tcp_shutdown();
    pub fn net_crc32le(data: *const u8, size: c_int) -> u32;
    pub fn net_crc32be(data: *const u8, size: c_int) -> u32;
    pub fn net_crc16ccitt(data: *const u8, size: c_int, start: u16) -> u16;
    pub fn net_multicast_add(mac: *const u8) -> c_int;
    pub fn net_mutlicast_del(mac: *const u8) -> c_int;
    pub fn net_multicast_check(mac: *const u8) -> c_int;
    pub fn net_multicast_init() -> c_int;
    pub fn net_multicast_shutdown();
    pub fn net_get_if_list() -> *mut netif_list;
    pub fn net_set_default(n: *mut netif_t) -> *mut netif_t;
    pub fn net_reg_device(device: *mut netif_t) -> c_int;
    pub fn net_unreg_device(device: *mut netif_t) -> c_int;
    pub fn net_init(ip: u32) -> c_int;
    pub fn net_shutdown();
}
