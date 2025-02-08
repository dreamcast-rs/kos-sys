// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

#[repr(C)]
pub enum pvr_palfmt_t {
    PVR_PAL_ARGB1555,
    PVR_PAL_RGB565,
    PVR_PAL_ARGB4444,
    PVR_PAL_ARGB8888,
}

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn pvr_set_pal_format(fmt: pvr_palfmt_t);
    #[link_name = "pvr_set_pal_entry_wrapper"]
    pub fn pvr_set_pal_entry(idx: u32, value: u32);
}
