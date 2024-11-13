use crate::prelude::*;

pub const BFONT_THIN_WIDTH: u32                 = 12;
pub const BFONT_WIDE_WIDTH: u32                 = BFONT_THIN_WIDTH * 2;
pub const BFONT_HEIGHT: u32                     = 24;

pub const JISX_0208_ROW_SIZE: u32               = 94;
pub const BFONT_NARROW_START: u32               = 0;
pub const BFONT_OVERBAR: u32                    = BFONT_NARROW_START;
pub const BFONT_ISO_8859_1_33_126: u32          = BFONT_NARROW_START +
                                                  (1*BFONT_THIN_WIDTH*BFONT_HEIGHT/8);
pub const BFONT_YEN: u32                        = BFONT_NARROW_START +
                                                  (95*BFONT_THIN_WIDTH*BFONT_HEIGHT/8);
pub const BFONT_ISO_8859_1_160_255: u32         = BFONT_NARROW_START +
                                                  (96*BFONT_THIN_WIDTH*BFONT_HEIGHT/8);


pub const BFONT_WIDE_START: u32                 = 288*BFONT_THIN_WIDTH*BFONT_HEIGHT/8;
pub const BFONT_JISX_0208_ROW1: u32             = BFONT_WIDE_START;
pub const BFONT_JISX_0208_ROW16: u32            = BFONT_WIDE_START +
                                                  (658*BFONT_WIDE_WIDTH*BFONT_HEIGHT/8);
pub const BFONT_JISX_0208_ROW48: u32            = BFONT_JISX_0208_ROW16 +
                                                  ((32*JISX_0208_ROW_SIZE) *
                                                   BFONT_WIDE_WIDTH*BFONT_HEIGHT/8);


pub const BFONT_DREAMCAST_SPECIFIC: u32         = BFONT_WIDE_START +
                                                  (7056*BFONT_WIDE_WIDTH*BFONT_HEIGHT/8);
pub const BFONT_CIRCLECOPYRIGHT: u32            = BFONT_DREAMCAST_SPECIFIC +
                                                  (0*BFONT_WIDE_WIDTH*BFONT_HEIGHT/8);
pub const BFONT_CIRCLER: u32                    = BFONT_DREAMCAST_SPECIFIC +
                                                  (1*BFONT_WIDE_WIDTH*BFONT_HEIGHT/8);
pub const BFONT_TRADEMARK: u32                  = BFONT_DREAMCAST_SPECIFIC +
                                                  (2*BFONT_WIDE_WIDTH*BFONT_HEIGHT/8);
pub const BFONT_UPARROW: u32                    = BFONT_DREAMCAST_SPECIFIC +
                                                  (3*BFONT_WIDE_WIDTH*BFONT_HEIGHT/8);
pub const BFONT_DOWNARROW: u32                  = BFONT_DREAMCAST_SPECIFIC +
                                                  (4*BFONT_WIDE_WIDTH*BFONT_HEIGHT/8);
pub const BFONT_LEFTARROW: u32                  = BFONT_DREAMCAST_SPECIFIC +
                                                  (5*BFONT_WIDE_WIDTH*BFONT_HEIGHT/8);
pub const BFONT_RIGHTARROW: u32                 = BFONT_DREAMCAST_SPECIFIC +
                                                  (6*BFONT_WIDE_WIDTH*BFONT_HEIGHT/8);
pub const BFONT_UPRIGHTARROW: u32               = BFONT_DREAMCAST_SPECIFIC +
                                                  (7*BFONT_WIDE_WIDTH*BFONT_HEIGHT/8);
pub const BFONT_DOWNRIGHTARROW: u32             = BFONT_DREAMCAST_SPECIFIC +
                                                  (8*BFONT_WIDE_WIDTH*BFONT_HEIGHT/8);
pub const BFONT_DOWNLEFTARROW: u32              = BFONT_DREAMCAST_SPECIFIC +
                                                  (9*BFONT_WIDE_WIDTH*BFONT_HEIGHT/8);
pub const BFONT_UPLEFTARROW: u32                = BFONT_DREAMCAST_SPECIFIC +
                                                  (10*BFONT_WIDE_WIDTH*BFONT_HEIGHT/8);
pub const BFONT_ABUTTON: u32                    = BFONT_DREAMCAST_SPECIFIC +
                                                  (11*BFONT_WIDE_WIDTH*BFONT_HEIGHT/8);
pub const BFONT_BBUTTON: u32                    = BFONT_DREAMCAST_SPECIFIC +
                                                  (12*BFONT_WIDE_WIDTH*BFONT_HEIGHT/8);
pub const BFONT_CBUTTON: u32                    = BFONT_DREAMCAST_SPECIFIC +
                                                  (13*BFONT_WIDE_WIDTH*BFONT_HEIGHT/8);
pub const BFONT_DBUTTON: u32                    = BFONT_DREAMCAST_SPECIFIC +
                                                  (14*BFONT_WIDE_WIDTH*BFONT_HEIGHT/8);
pub const BFONT_XBUTTON: u32                    = BFONT_DREAMCAST_SPECIFIC +
                                                  (15*BFONT_WIDE_WIDTH*BFONT_HEIGHT/8);
pub const BFONT_YBUTTON: u32                    = BFONT_DREAMCAST_SPECIFIC +
                                                  (16*BFONT_WIDE_WIDTH*BFONT_HEIGHT/8);
pub const BFONT_ZBUTTON: u32                    = BFONT_DREAMCAST_SPECIFIC +
                                                  (17*BFONT_WIDE_WIDTH*BFONT_HEIGHT/8);
pub const BFONT_LTRIGGER: u32                   = BFONT_DREAMCAST_SPECIFIC +
                                                  (18*BFONT_WIDE_WIDTH*BFONT_HEIGHT/8);
pub const BFONT_RTRIGGER: u32                   = BFONT_DREAMCAST_SPECIFIC +
                                                  (19*BFONT_WIDE_WIDTH*BFONT_HEIGHT/8);
pub const BFONT_STARTBUTTON: u32                = BFONT_DREAMCAST_SPECIFIC + 
                                                  (20*BFONT_WIDE_WIDTH*BFONT_HEIGHT/8);
pub const BFONT_VMUICON: u32                    = BFONT_DREAMCAST_SPECIFIC + 
                                                  (21*BFONT_WIDE_WIDTH*BFONT_HEIGHT/8);

pub const BFONT_ICON_DIMEN: u32                 = 32;
pub const BFONT_VMU_DREAMCAST_SPECIFIC: u32     = BFONT_DREAMCAST_SPECIFIC +
                                                  (22*BFONT_WIDE_WIDTH*BFONT_HEIGHT/8);

pub const BFONT_ICON_INVALID_VMU: u8            = 0x00;
pub const BFONT_ICON_HOURGLASS_ONE: u8          = 0x01;
pub const BFONT_ICON_HOURGLASS_TWO: u8          = 0x02;
pub const BFONT_ICON_HOURGLASS_THREE: u8        = 0x03;
pub const BFONT_ICON_HOURGLASS_FOUR: u8         = 0x04;
pub const BFONT_ICON_VMUICON: u8                = 0x05;
pub const BFONT_ICON_EARTH: u8                  = 0x06;
pub const BFONT_ICON_SATURN: u8                 = 0x07;
pub const BFONT_ICON_QUARTER_MOON: u8           = 0x08;
pub const BFONT_ICON_LAUGHING_FACE: u8          = 0x09;
pub const BFONT_ICON_SMILING_FACE: u8           = 0x0A;
pub const BFONT_ICON_CASUAL_FACE: u8            = 0x0B;
pub const BFONT_ICON_ANGRY_FACE: u8             = 0x0C;
pub const BFONT_ICON_COW: u8                    = 0x0D;
pub const BFONT_ICON_HORSE: u8                  = 0x0E;
pub const BFONT_ICON_RABBIT: u8                 = 0x0F;
pub const BFONT_ICON_CAT: u8                    = 0x10;
pub const BFONT_ICON_CHICK: u8                  = 0x11;
pub const BFONT_ICON_LION: u8                   = 0x12;
pub const BFONT_ICON_MONKEY: u8                 = 0x13;
pub const BFONT_ICON_PANDA: u8                  = 0x14;
pub const BFONT_ICON_BEAR: u8                   = 0x15;
pub const BFONT_ICON_PIG: u8                    = 0x16;
pub const BFONT_ICON_DOG: u8                    = 0x17;
pub const BFONT_ICON_FISH: u8                   = 0x18;
pub const BFONT_ICON_OCTOPUS: u8                = 0x19;
pub const BFONT_ICON_SQUID: u8                  = 0x1A;
pub const BFONT_ICON_WHALE: u8                  = 0x1B;
pub const BFONT_ICON_CRAB: u8                   = 0x1C;
pub const BFONT_ICON_BUTTERFLY: u8              = 0x1D;
pub const BFONT_ICON_LADYBUG: u8                = 0x1E;
pub const BFONT_ICON_ANGLER_FISH: u8            = 0x1F;
pub const BFONT_ICON_PENGUIN: u8                = 0x20;
pub const BFONT_ICON_CHERRIES: u8               = 0x21;
pub const BFONT_ICON_TULIP: u8                  = 0x22;
pub const BFONT_ICON_LEAF: u8                   = 0x23;
pub const BFONT_ICON_SAKURA: u8                 = 0x24;
pub const BFONT_ICON_APPLE: u8                  = 0x25;
pub const BFONT_ICON_ICECREAM: u8               = 0x26;
pub const BFONT_ICON_CACTUS: u8                 = 0x27;
pub const BFONT_ICON_PIANO: u8                  = 0x28;
pub const BFONT_ICON_GUITAR: u8                 = 0x29;
pub const BFONT_ICON_EIGHTH_NOTE: u8            = 0x2A;
pub const BFONT_ICON_TREBLE_CLEF: u8            = 0x2B;
pub const BFONT_ICON_BOAT: u8                   = 0x2C;
pub const BFONT_ICON_CAR: u8                    = 0x2D;
pub const BFONT_ICON_HELMET: u8                 = 0x2E;
pub const BFONT_ICON_MOTORCYCLE: u8             = 0x2F;
pub const BFONT_ICON_VAN: u8                    = 0x30;
pub const BFONT_ICON_TRUCK: u8                  = 0x31;
pub const BFONT_ICON_CLOCK: u8                  = 0x32;
pub const BFONT_ICON_TELEPHONE: u8              = 0x33;
pub const BFONT_ICON_PENCIL: u8                 = 0x34;
pub const BFONT_ICON_CUP: u8                    = 0x35;
pub const BFONT_ICON_SILVERWARE: u8             = 0x36;
pub const BFONT_ICON_HOUSE: u8                  = 0x37;
pub const BFONT_ICON_BELL: u8                   = 0x38;
pub const BFONT_ICON_CROWN: u8                  = 0x39;
pub const BFONT_ICON_SOCK: u8                   = 0x3A;
pub const BFONT_ICON_CAKE: u8                   = 0x3B;
pub const BFONT_ICON_KEY: u8                    = 0x3C;
pub const BFONT_ICON_BOOK: u8                   = 0x3D;
pub const BFONT_ICON_BASEBALL: u8               = 0x3E;
pub const BFONT_ICON_SOCCER: u8                 = 0x3F;
pub const BFONT_ICON_BULB: u8                   = 0x40;
pub const BFONT_ICON_TEDDY_BEAR: u8             = 0x41;
pub const BFONT_ICON_BOW_TIE: u8                = 0x42;
pub const BFONT_ICON_BOW_ARROW: u8              = 0x43;
pub const BFONT_ICON_SNOWMAN: u8                = 0x44;
pub const BFONT_ICON_LIGHTNING: u8              = 0x45;
pub const BFONT_ICON_SUN: u8                    = 0x46;
pub const BFONT_ICON_CLOUD: u8                  = 0x47;
pub const BFONT_ICON_UMBRELLA: u8               = 0x48;
pub const BFONT_ICON_ONE_STAR: u8               = 0x49;
pub const BFONT_ICON_TWO_STARS: u8              = 0x4A;
pub const BFONT_ICON_THREE_STARS: u8            = 0x4B;
pub const BFONT_ICON_FOUR_STARS: u8             = 0x4C;
pub const BFONT_ICON_HEART: u8                  = 0x4D;
pub const BFONT_ICON_DIAMOND: u8                = 0x4E;
pub const BFONT_ICON_SPADE: u8                  = 0x4F;
pub const BFONT_ICON_CLUB: u8                   = 0x50;
pub const BFONT_ICON_JACK: u8                   = 0x51;
pub const BFONT_ICON_QUEEN: u8                  = 0x52;
pub const BFONT_ICON_KING: u8                   = 0x53;
pub const BFONT_ICON_JOKER: u8                  = 0x54;
pub const BFONT_ICON_ISLAND: u8                 = 0x55;
pub const BFONT_ICON_0: u8                      = 0x56;
pub const BFONT_ICON_1: u8                      = 0x57;
pub const BFONT_ICON_2: u8                      = 0x58;
pub const BFONT_ICON_3: u8                      = 0x59;
pub const BFONT_ICON_4: u8                      = 0x5A;
pub const BFONT_ICON_5: u8                      = 0x5B;
pub const BFONT_ICON_6: u8                      = 0x5C;
pub const BFONT_ICON_7: u8                      = 0x5D;
pub const BFONT_ICON_8: u8                      = 0x5E;
pub const BFONT_ICON_9: u8                      = 0x5F;
pub const BFONT_ICON_A: u8                      = 0x60;
pub const BFONT_ICON_B: u8                      = 0x61;
pub const BFONT_ICON_C: u8                      = 0x62;
pub const BFONT_ICON_D: u8                      = 0x63;
pub const BFONT_ICON_E: u8                      = 0x64;
pub const BFONT_ICON_F: u8                      = 0x65;
pub const BFONT_ICON_G: u8                      = 0x66;
pub const BFONT_ICON_H: u8                      = 0x67;
pub const BFONT_ICON_I: u8                      = 0x68;
pub const BFONT_ICON_J: u8                      = 0x69;
pub const BFONT_ICON_K: u8                      = 0x6A;
pub const BFONT_ICON_L: u8                      = 0x6B;
pub const BFONT_ICON_M: u8                      = 0x6C;
pub const BFONT_ICON_N: u8                      = 0x6D;
pub const BFONT_ICON_O: u8                      = 0x6E;
pub const BFONT_ICON_P: u8                      = 0x6F;
pub const BFONT_ICON_Q: u8                      = 0x70;
pub const BFONT_ICON_R: u8                      = 0x71;
pub const BFONT_ICON_S: u8                      = 0x72;
pub const BFONT_ICON_T: u8                      = 0x73;
pub const BFONT_ICON_U: u8                      = 0x74;
pub const BFONT_ICON_V: u8                      = 0x75;
pub const BFONT_ICON_W: u8                      = 0x76;
pub const BFONT_ICON_X: u8                      = 0x77;
pub const BFONT_ICON_Y: u8                      = 0x78;
pub const BFONT_ICON_Z: u8                      = 0x79;
pub const BFONT_ICON_CHECKER_BOARD: u8          = 0x7A;
pub const BFONT_ICON_GRID: u8                   = 0x7B;
pub const BFONT_ICON_LIGHT_GRAY: u8             = 0x7C;
pub const BFONT_ICON_DIAG_GRID: u8              = 0x7D;
pub const BFONT_ICON_PACMAN_GRID: u8            = 0x7E;
pub const BFONT_ICON_DARK_GRAY: u8              = 0x7F;
pub const BFONT_ICON_EMBROIDERY: u8             = 0x80;

pub const BFONT_CODE_ISO8859_1: u8              = 0;
pub const BFONT_CODE_EUC: u8                    = 1;
pub const BFONT_CODE_SJIS: u8                   = 2;
pub const BFONT_CODE_RAW: u8                    = 3;

extern "C" {
    pub fn bfont_set_foreground_color(c: u32) -> u32;
    pub fn bfont_set_background_color(c: u32) -> u32;
    pub fn bfont_set_32bit_mode(on: c_int) -> c_int;
    pub fn bfont_set_encoding(enc: u8);
    pub fn bfont_find_char(ch: u32) -> *mut u8;
    pub fn bfont_find_char_jp(ch: u32) -> *mut u8;
    pub fn bfont_find_char_jp_half(ch: u32) -> *mut u8;
    pub fn bfont_draw_ex(buffer: *mut u8, bufwidth: u32, fg: u32, bg: u32, bpp: u8,
                         opaque: u8, c: u32, wide: u8, iskana: u8) -> c_uchar;
    pub fn bfont_draw(buffer: *mut c_void, bufwidth: u32, opaque: u8, c: u32) -> c_uchar;
    pub fn bfont_draw_thin(buffer: *mut c_void, bufwidth: u32, opaque: u8,
                           c: u32, iskana: u8) -> c_uchar;
    pub fn bfont_draw_wide(buffer: *mut c_void, bufwidth: u32, opaque: u8,
                           c: u32) -> c_uchar;
    pub fn bfont_draw_str_ex(b: *mut c_void, width: u32, fg: u32, bg: u32, bpp: u8,
                             opaque: u8, st: *const c_char);
    pub fn bfont_draw_str(b: *mut c_void, width: u32, opaque: u8, str: *const c_char);
    pub fn bfont_find_icon(icon: u8) -> *mut u8;
}