// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

use crate::arch::types::ptr_t;
use crate::os::library::klibrary_t;

#[repr(C)]
pub struct elf_hdr_t {
    ident:              [u8; 16],
    r#type:             u16,
    machine:            u16,
    version:            u32,
    entry:              u32,
    phoff:              u32,
    shoff:              u32,
    flags:              u32,
    ehsize:             u16,
    phentsize:          u16,
    phnum:              u16,
    shentsize:          u16,
    shnum:              u16,
    shstrndx:           u16,
}

pub const EM_386: u16           = 3;
pub const EM_ARM: u16           = 40;
pub const EM_SH: u16            = 42;

pub const SHT_NULL: u32         = 0;
pub const SHT_PROGBITS: u32     = 1;
pub const SHT_SYMTAB: u32       = 2;
pub const SHT_STRTAB: u32       = 3;
pub const SHT_RELA: u32         = 4;
pub const SHT_HASH: u32         = 5;
pub const SHT_DYNAMIC: u32      = 6;
pub const SHT_NOTE: u32         = 7;
pub const SHT_NOBITS: u32       = 8;
pub const SHT_REL: u32          = 9;
pub const SHT_SHLIB: u32        = 10;
pub const SHT_DYNSYM: u32       = 11;
pub const SHT_LOPROC: u32       = 0x70000000;
pub const SHT_HIPROC: u32       = 0x7fffffff;
pub const SHT_LOUSER: u32       = 0x80000000;
pub const SHT_HIUSER: u32       = 0xffffffff;

pub const SHF_WRITE: u32        = 1;
pub const SHF_ALLOC: u32        = 2;
pub const SHF_EXECINSTR: u32    = 4;
pub const SHF_MASKPROC: u32     = 0xf0000000;

pub const SHN_UNDEF: u16        = 0;
pub const SHN_ABS: u16          = 0xfff1;

#[repr(C)]
pub struct elf_shdr_t {
    name:               u32,
    r#type:             u32,
    flags:              u32,
    addr:               u32,
    offset:             u32,
    size:               u32,
    link:               u32,
    info:               u32,
    addralign:          u32,
    entsize:            u32,
}

pub const STB_LOCAL: u8         = 0;
pub const STB_GLOBAL: u8        = 1;
pub const STB_WEAK: u8          = 2;

pub const STT_NOTYPE: u8        = 0;
pub const STT_OBJECT: u8        = 1;
pub const STT_FUNC: u8          = 2;
pub const STT_SECTION: u8       = 3;
pub const STT_FILE: u8          = 4;

#[repr(C)]
pub struct elf_sym_t {
    name:               u32,
    value:              u32,
    size:               u32,
    info:               u8,
    other:              u8,
    shndx:              u16,
}

#[macro_export]
macro_rules! ELF32_ST_BIND {
    ($info:expr) => {
        ($info >> 4)
    };
}

#[macro_export]
macro_rules! ELF32_ST_TYPE {
    ($info:expr) => {
        ($info & 0xf)
    };
}

#[repr(C)]
pub struct elf_rela_t {
    offset:             u32,
    info:               u32,
    addend:             i32,
}

#[repr(C)]
pub struct elf_rel_t {
    offset:             u32,
    info:               u32,
}

pub const R_SH_DIR32: u8        = 1;
pub const R_386_32: u8          = 1;
pub const R_386_PC32: u8        = 2;

#[macro_export]
macro_rules! ELF32_R_SYM {
    ($i:expr) => {
        ($i >> 8)
    };
}

#[macro_export]
macro_rules! ELF32_R_TYPE {
    ($i:expr) => {
        ($i as u8)
    };
}

#[repr(C)]
pub struct elf_prog_t {
    data:               *mut c_void,
    size:               u32,
    lib_get_name:       ptr_t,
    lib_get_version:    ptr_t,
    lib_open:           ptr_t,
    lib_close:          ptr_t,
    r#fn:               [c_char; 256],
}

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn elf_load(r#fn: *const c_char, shell: *mut klibrary_t, out: *mut elf_prog_t) -> c_int;
    pub fn elf_free(prog: *mut elf_prog_t);
}
