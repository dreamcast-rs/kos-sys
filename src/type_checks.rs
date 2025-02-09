// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

// FIXME: Check signedness, too

/* Verify core::ffi uses expected sizes and alignments */
const _: () = assert!(size_of::<c_char>() == size_of::<i8>());
const _: () = assert!(align_of::<c_char>() == align_of::<i8>());

const _: () = assert!(size_of::<c_schar>() == size_of::<i8>());
const _: () = assert!(align_of::<c_schar>() == align_of::<i8>());

const _: () = assert!(size_of::<c_uchar>() == size_of::<u8>());
const _: () = assert!(align_of::<c_uchar>() == align_of::<u8>());

const _: () = assert!(size_of::<c_short>() == size_of::<i16>());
const _: () = assert!(align_of::<c_short>() == align_of::<i16>());

const _: () = assert!(size_of::<c_ushort>() == size_of::<u16>());
const _: () = assert!(align_of::<c_ushort>() == align_of::<u16>());

const _: () = assert!(size_of::<c_int>() == size_of::<i32>());
const _: () = assert!(align_of::<c_int>() == align_of::<i32>());

const _: () = assert!(size_of::<c_uint>() == size_of::<u32>());
const _: () = assert!(align_of::<c_uint>() == align_of::<u32>());

const _: () = assert!(size_of::<c_long>() == size_of::<i32>());
const _: () = assert!(align_of::<c_long>() == align_of::<i32>());

const _: () = assert!(size_of::<c_ulong>() == size_of::<u32>());
const _: () = assert!(align_of::<c_ulong>() == align_of::<u32>());

const _: () = assert!(size_of::<c_size_t>() == size_of::<usize>());
const _: () = assert!(align_of::<c_size_t>() == align_of::<usize>());

const _: () = assert!(size_of::<c_ssize_t>() == size_of::<isize>());
const _: () = assert!(align_of::<c_ssize_t>() == align_of::<isize>());

const _: () = assert!(size_of::<c_ptrdiff_t>() == size_of::<isize>());
const _: () = assert!(align_of::<c_ptrdiff_t>() == align_of::<isize>());

const _: () = assert!(size_of::<c_longlong>() == size_of::<i64>());
const _: () = assert!(align_of::<c_longlong>() == align_of::<i64>());

const _: () = assert!(size_of::<c_longlong>() == size_of::<u64>());
const _: () = assert!(align_of::<c_longlong>() == align_of::<u64>());

const _: () = assert!(size_of::<c_float>() == size_of::<f32>());
const _: () = assert!(align_of::<c_float>() == align_of::<f32>());

const _: () = assert!(size_of::<c_double>() == size_of::<f64>());
const _: () = assert!(align_of::<c_double>() == align_of::<f64>());

/* Verify libc uses expected sizes and alignments */
const _: () = assert!(size_of::<libc::c_char>() == size_of::<i8>());
const _: () = assert!(align_of::<libc::c_char>() == align_of::<i8>());

const _: () = assert!(size_of::<libc::c_schar>() == size_of::<i8>());
const _: () = assert!(align_of::<libc::c_schar>() == align_of::<i8>());

const _: () = assert!(size_of::<libc::c_uchar>() == size_of::<u8>());
const _: () = assert!(align_of::<libc::c_uchar>() == align_of::<u8>());

const _: () = assert!(size_of::<libc::c_short>() == size_of::<i16>());
const _: () = assert!(align_of::<libc::c_short>() == align_of::<i16>());

const _: () = assert!(size_of::<libc::c_ushort>() == size_of::<u16>());
const _: () = assert!(align_of::<libc::c_ushort>() == align_of::<u16>());

const _: () = assert!(size_of::<libc::c_int>() == size_of::<i32>());
const _: () = assert!(align_of::<libc::c_int>() == align_of::<i32>());

const _: () = assert!(size_of::<libc::c_uint>() == size_of::<u32>());
const _: () = assert!(align_of::<libc::c_uint>() == align_of::<u32>());

const _: () = assert!(size_of::<libc::c_long>() == size_of::<i32>());
const _: () = assert!(align_of::<libc::c_long>() == align_of::<i32>());

const _: () = assert!(size_of::<libc::c_ulong>() == size_of::<u32>());
const _: () = assert!(align_of::<libc::c_ulong>() == align_of::<u32>());

const _: () = assert!(size_of::<libc::c_size_t>() == size_of::<usize>());
const _: () = assert!(align_of::<libc::c_size_t>() == align_of::<usize>());

const _: () = assert!(size_of::<libc::c_ssize_t>() == size_of::<isize>());
const _: () = assert!(align_of::<libc::c_ssize_t>() == align_of::<isize>());

/* FIXME: Add this to libc crate
const _: () = assert!(size_of::<libc::c_ptrdiff_t>() == size_of::<isize>());
const _: () = assert!(align_of::<libc::c_ptrdiff_t>() == align_of::<isize>());
*/

const _: () = assert!(size_of::<libc::c_longlong>() == size_of::<i64>());
const _: () = assert!(align_of::<libc::c_longlong>() == align_of::<i64>());

const _: () = assert!(size_of::<libc::c_longlong>() == size_of::<u64>());
const _: () = assert!(align_of::<libc::c_longlong>() == align_of::<u64>());

const _: () = assert!(size_of::<libc::c_float>() == size_of::<f32>());
const _: () = assert!(align_of::<libc::c_float>() == align_of::<f32>());

const _: () = assert!(size_of::<libc::c_double>() == size_of::<f64>());
const _: () = assert!(align_of::<libc::c_double>() == align_of::<f64>());
