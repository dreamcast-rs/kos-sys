// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub static dbgio: crate::os::dbgio::dbgio_handler_t;
    pub fn scif_set_parameters(baud: c_int, fifo: c_int);
    pub fn scif_set_irq_usage(on: c_int) -> c_int;
    pub fn scif_detected() -> c_int;
    pub fn scif_init() -> c_int;
    pub fn scif_shutdown() -> c_int;
    pub fn scif_read() -> c_int;
    pub fn scif_write(c: c_int) -> c_int;
    pub fn scif_flush() -> c_int;
    pub fn scif_write_buffer(data: *const u8, len: c_int, xlat: c_int) -> c_int;
    pub fn scif_read_buffer(data: *mut u8, len: c_int) -> c_int;
    pub fn scif_spi_init() -> c_int;
    pub fn scif_spi_shutdown() -> c_int;
    pub fn scif_spi_set_cs(v: c_int);
    pub fn scif_spi_rw_byte(b: u8) -> u8;
    pub fn scif_spi_slow_rw_byte(b: u8) -> u8;
    pub fn scif_spi_write_byte(b: u8);
    pub fn scif_spi_read_byte() -> u8;
    pub fn scif_spi_read_data(buffer: *mut u8, len: c_size_t);
}
