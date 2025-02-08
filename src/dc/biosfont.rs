// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

pub const BFONT_THIN_WIDTH: u32                 = 12;
pub const BFONT_WIDE_WIDTH: u32                 = BFONT_THIN_WIDTH * 2;
pub const BFONT_HEIGHT: u32                     = 24;

pub const BFONT_BYTES_PER_CHAR: u32             = BFONT_THIN_WIDTH * BFONT_HEIGHT / 8;
pub const BFONT_BYTES_PER_WIDE_CHAR: u32        = BFONT_WIDE_WIDTH * BFONT_HEIGHT / 8;

pub const BFONT_NARROW_START: u32               = 0;
pub const BFONT_OVERBAR: u32                    = BFONT_NARROW_START;
pub const BFONT_ISO_8859_1_33_126: u32          = BFONT_NARROW_START +
                                                  (1*BFONT_BYTES_PER_CHAR);
pub const BFONT_YEN: u32                        = BFONT_NARROW_START +
                                                  (95*BFONT_BYTES_PER_CHAR);
pub const BFONT_ISO_8859_1_160_255: u32         = BFONT_NARROW_START +
                                                  (96*BFONT_BYTES_PER_CHAR);

pub const JISX_0208_ROW_SIZE: u32               = 94;

pub const BFONT_WIDE_START: u32                 = 288*BFONT_BYTES_PER_CHAR;
pub const BFONT_JISX_0208_ROW1: u32             = BFONT_WIDE_START;
pub const BFONT_JISX_0208_ROW16: u32            = BFONT_WIDE_START +
                                                  (658*BFONT_BYTES_PER_WIDE_CHAR);
pub const BFONT_JISX_0208_ROW48: u32            = BFONT_JISX_0208_ROW16 +
                                                  ((32*JISX_0208_ROW_SIZE) *
                                                   BFONT_BYTES_PER_WIDE_CHAR);


pub const BFONT_DREAMCAST_SPECIFIC: u32         = BFONT_WIDE_START +
                                                  (7056*BFONT_BYTES_PER_WIDE_CHAR);

macro_rules! BFONT_DC_ICON {
    ($offset:expr) => {
        BFONT_DREAMCAST_SPECIFIC + ($offset * BFONT_BYTES_PER_WIDE_CHAR)
    };
}

pub const BFONT_CIRCLECOPYRIGHT: u32            = BFONT_DC_ICON!(0);
pub const BFONT_CIRCLER: u32                    = BFONT_DC_ICON!(1);
pub const BFONT_TRADEMARK: u32                  = BFONT_DC_ICON!(2);
pub const BFONT_UPARROW: u32                    = BFONT_DC_ICON!(3);
pub const BFONT_DOWNARROW: u32                  = BFONT_DC_ICON!(4);
pub const BFONT_LEFTARROW: u32                  = BFONT_DC_ICON!(5);
pub const BFONT_RIGHTARROW: u32                 = BFONT_DC_ICON!(6);
pub const BFONT_UPRIGHTARROW: u32               = BFONT_DC_ICON!(7);
pub const BFONT_DOWNRIGHTARROW: u32             = BFONT_DC_ICON!(8);
pub const BFONT_DOWNLEFTARROW: u32              = BFONT_DC_ICON!(9);
pub const BFONT_UPLEFTARROW: u32                = BFONT_DC_ICON!(10);
pub const BFONT_ABUTTON: u32                    = BFONT_DC_ICON!(11);
pub const BFONT_BBUTTON: u32                    = BFONT_DC_ICON!(12);
pub const BFONT_CBUTTON: u32                    = BFONT_DC_ICON!(13);
pub const BFONT_DBUTTON: u32                    = BFONT_DC_ICON!(14);
pub const BFONT_XBUTTON: u32                    = BFONT_DC_ICON!(15);
pub const BFONT_YBUTTON: u32                    = BFONT_DC_ICON!(16);
pub const BFONT_ZBUTTON: u32                    = BFONT_DC_ICON!(17);
pub const BFONT_LTRIGGER: u32                   = BFONT_DC_ICON!(18);
pub const BFONT_RTRIGGER: u32                   = BFONT_DC_ICON!(19);
pub const BFONT_STARTBUTTON: u32                = BFONT_DC_ICON!(20);
pub const BFONT_VMUICON: u32                    = BFONT_DC_ICON!(21);

pub const BFONT_ICON_DIMEN: u32                 = 32;
pub const BFONT_VMU_DREAMCAST_SPECIFIC: u32     = BFONT_DREAMCAST_SPECIFIC +
                                                  (22*BFONT_BYTES_PER_WIDE_CHAR);

#[repr(C)]
pub enum bfont_vmu_icon_t {
    BFONT_ICON_INVALID_VMU                      = 0x00,
    BFONT_ICON_HOURGLASS_ONE                    = 0x01,
    BFONT_ICON_HOURGLASS_TWO                    = 0x02,
    BFONT_ICON_HOURGLASS_THREE                  = 0x03,
    BFONT_ICON_HOURGLASS_FOUR                   = 0x04,
    BFONT_ICON_VMUICON                          = 0x05,
    BFONT_ICON_EARTH                            = 0x06,
    BFONT_ICON_SATURN                           = 0x07,
    BFONT_ICON_QUARTER_MOON                     = 0x08,
    BFONT_ICON_LAUGHING_FACE                    = 0x09,
    BFONT_ICON_SMILING_FACE                     = 0x0A,
    BFONT_ICON_CASUAL_FACE                      = 0x0B,
    BFONT_ICON_ANGRY_FACE                       = 0x0C,
    BFONT_ICON_COW                              = 0x0D,
    BFONT_ICON_HORSE                            = 0x0E,
    BFONT_ICON_RABBIT                           = 0x0F,
    BFONT_ICON_CAT                              = 0x10,
    BFONT_ICON_CHICK                            = 0x11,
    BFONT_ICON_LION                             = 0x12,
    BFONT_ICON_MONKEY                           = 0x13,
    BFONT_ICON_PANDA                            = 0x14,
    BFONT_ICON_BEAR                             = 0x15,
    BFONT_ICON_PIG                              = 0x16,
    BFONT_ICON_DOG                              = 0x17,
    BFONT_ICON_FISH                             = 0x18,
    BFONT_ICON_OCTOPUS                          = 0x19,
    BFONT_ICON_SQUID                            = 0x1A,
    BFONT_ICON_WHALE                            = 0x1B,
    BFONT_ICON_CRAB                             = 0x1C,
    BFONT_ICON_BUTTERFLY                        = 0x1D,
    BFONT_ICON_LADYBUG                          = 0x1E,
    BFONT_ICON_ANGLER_FISH                      = 0x1F,
    BFONT_ICON_PENGUIN                          = 0x20,
    BFONT_ICON_CHERRIES                         = 0x21,
    BFONT_ICON_TULIP                            = 0x22,
    BFONT_ICON_LEAF                             = 0x23,
    BFONT_ICON_SAKURA                           = 0x24,
    BFONT_ICON_APPLE                            = 0x25,
    BFONT_ICON_ICECREAM                         = 0x26,
    BFONT_ICON_CACTUS                           = 0x27,
    BFONT_ICON_PIANO                            = 0x28,
    BFONT_ICON_GUITAR                           = 0x29,
    BFONT_ICON_EIGHTH_NOTE                      = 0x2A,
    BFONT_ICON_TREBLE_CLEF                      = 0x2B,
    BFONT_ICON_BOAT                             = 0x2C,
    BFONT_ICON_CAR                              = 0x2D,
    BFONT_ICON_HELMET                           = 0x2E,
    BFONT_ICON_MOTORCYCLE                       = 0x2F,
    BFONT_ICON_VAN                              = 0x30,
    BFONT_ICON_TRUCK                            = 0x31,
    BFONT_ICON_CLOCK                            = 0x32,
    BFONT_ICON_TELEPHONE                        = 0x33,
    BFONT_ICON_PENCIL                           = 0x34,
    BFONT_ICON_CUP                              = 0x35,
    BFONT_ICON_SILVERWARE                       = 0x36,
    BFONT_ICON_HOUSE                            = 0x37,
    BFONT_ICON_BELL                             = 0x38,
    BFONT_ICON_CROWN                            = 0x39,
    BFONT_ICON_SOCK                             = 0x3A,
    BFONT_ICON_CAKE                             = 0x3B,
    BFONT_ICON_KEY                              = 0x3C,
    BFONT_ICON_BOOK                             = 0x3D,
    BFONT_ICON_BASEBALL                         = 0x3E,
    BFONT_ICON_SOCCER                           = 0x3F,
    BFONT_ICON_BULB                             = 0x40,
    BFONT_ICON_TEDDY_BEAR                       = 0x41,
    BFONT_ICON_BOW_TIE                          = 0x42,
    BFONT_ICON_BOW_ARROW                        = 0x43,
    BFONT_ICON_SNOWMAN                          = 0x44,
    BFONT_ICON_LIGHTNING                        = 0x45,
    BFONT_ICON_SUN                              = 0x46,
    BFONT_ICON_CLOUD                            = 0x47,
    BFONT_ICON_UMBRELLA                         = 0x48,
    BFONT_ICON_ONE_STAR                         = 0x49,
    BFONT_ICON_TWO_STARS                        = 0x4A,
    BFONT_ICON_THREE_STARS                      = 0x4B,
    BFONT_ICON_FOUR_STARS                       = 0x4C,
    BFONT_ICON_HEART                            = 0x4D,
    BFONT_ICON_DIAMOND                          = 0x4E,
    BFONT_ICON_SPADE                            = 0x4F,
    BFONT_ICON_CLUB                             = 0x50,
    BFONT_ICON_JACK                             = 0x51,
    BFONT_ICON_QUEEN                            = 0x52,
    BFONT_ICON_KING                             = 0x53,
    BFONT_ICON_JOKER                            = 0x54,
    BFONT_ICON_ISLAND                           = 0x55,
    BFONT_ICON_0                                = 0x56,
    BFONT_ICON_1                                = 0x57,
    BFONT_ICON_2                                = 0x58,
    BFONT_ICON_3                                = 0x59,
    BFONT_ICON_4                                = 0x5A,
    BFONT_ICON_5                                = 0x5B,
    BFONT_ICON_6                                = 0x5C,
    BFONT_ICON_7                                = 0x5D,
    BFONT_ICON_8                                = 0x5E,
    BFONT_ICON_9                                = 0x5F,
    BFONT_ICON_A                                = 0x60,
    BFONT_ICON_B                                = 0x61,
    BFONT_ICON_C                                = 0x62,
    BFONT_ICON_D                                = 0x63,
    BFONT_ICON_E                                = 0x64,
    BFONT_ICON_F                                = 0x65,
    BFONT_ICON_G                                = 0x66,
    BFONT_ICON_H                                = 0x67,
    BFONT_ICON_I                                = 0x68,
    BFONT_ICON_J                                = 0x69,
    BFONT_ICON_K                                = 0x6A,
    BFONT_ICON_L                                = 0x6B,
    BFONT_ICON_M                                = 0x6C,
    BFONT_ICON_N                                = 0x6D,
    BFONT_ICON_O                                = 0x6E,
    BFONT_ICON_P                                = 0x6F,
    BFONT_ICON_Q                                = 0x70,
    BFONT_ICON_R                                = 0x71,
    BFONT_ICON_S                                = 0x72,
    BFONT_ICON_T                                = 0x73,
    BFONT_ICON_U                                = 0x74,
    BFONT_ICON_V                                = 0x75,
    BFONT_ICON_W                                = 0x76,
    BFONT_ICON_X                                = 0x77,
    BFONT_ICON_Y                                = 0x78,
    BFONT_ICON_Z                                = 0x79,
    BFONT_ICON_CHECKER_BOARD                    = 0x7A,
    BFONT_ICON_GRID                             = 0x7B,
    BFONT_ICON_LIGHT_GRAY                       = 0x7C,
    BFONT_ICON_DIAG_GRID                        = 0x7D,
    BFONT_ICON_PACMAN_GRID                      = 0x7E,
    BFONT_ICON_DARK_GRAY                        = 0x7F,
    BFONT_ICON_EMBROIDERY                       = 0x80,
}

#[repr(C)]
pub enum bfont_code_t {
    BFONT_CODE_ISO8859_1                        = 0,
    BFONT_CODE_EUC                              = 1,
    BFONT_CODE_SJIS                             = 2,
    BFONT_CODE_RAW                              = 3,
}

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn bfont_set_foreground_color(c: u32) -> u32;
    pub fn bfont_set_background_color(c: u32) -> u32;
    pub fn bfont_set_encoding(enc: u8);
    pub fn bfont_find_char(ch: u32) -> *mut u8;
    pub fn bfont_find_char_jp(ch: u32) -> *mut u8;
    pub fn bfont_find_char_jp_half(ch: u32) -> *mut u8;
    pub fn bfont_find_icon(icon: bfont_vmu_icon_t) -> *mut u8;
    pub fn bfont_draw_ex(
        buffer: *mut c_void,
        bufwidth: u32,
        fg: u32,
        bg: u32,
        bpp: u8,
        opaque: bool,
        c: u32,
        wide: bool,
        iskana: bool,
    ) -> c_size_t;
    pub fn bfont_draw(buffer: *mut c_void, bufwidth: u32, opaque: bool, c: u32) -> c_size_t;
    pub fn bfont_draw_thin(
        buffer: *mut c_void,
        bufwidth: u32,
        opaque: bool,
        c: u32,
        iskana: bool,
    ) -> c_size_t;
    pub fn bfont_draw_wide(buffer: *mut c_void, bufwidth: u32, opaque: bool, c: u32) -> c_size_t;
    pub fn bfont_draw_str_ex(
        b: *mut c_void,
        width: u32,
        fg: u32,
        bg: u32,
        bpp: u8,
        opaque: bool,
        str: *const c_char,
    );
    pub fn bfont_draw_str_ex_fmt(
        b: *mut c_void,
        width: u32,
        fg: u32,
        bg: u32,
        bpp: u8,
        opaque: bool,
        fmt: *const c_char,
        ...
    );
    pub fn bfont_draw_str_ex_vfmt(
        b: *mut c_void,
        width: u32,
        fg: u32,
        bg: u32,
        bpp: u8,
        opaque: bool,
        fmt: *const c_char,
        var_args: *mut VaList,
    );
    pub fn bfont_draw_str(b: *mut c_void, width: u32, opaque: bool, str: *const c_char);
    pub fn bfont_draw_str_fmt(b: *mut c_void, width: u32, opaque: bool, fmt: *const c_char, ...);
    pub fn bfont_draw_str_vram_vfmt(
        x: u32,
        y: u32,
        fg: u32,
        bg: u32,
        opaque: bool,
        fmt: *const c_char,
        var_args: *mut VaList,
    );
    pub fn bfont_draw_str_vram_fmt(x: u32, y: u32, opaque: bool, fmt: *const c_char, ...);
}
