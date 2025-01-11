// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

#[macro_export]
macro_rules! BIT {
    ($bit:expr) => {
        1u32 << $bit
    };
}

#[macro_export]
macro_rules! GENMASK {
    ($h:expr, $l:expr) => {
        ((0xffffffff << $l) & (0xffffffff >> (31 - $h)))
    };
}

#[macro_export]
macro_rules! FIELD_GET {
    ($var:expr, $field:expr) => {
        (($var & $field) >> $field.trailing_zeros())
    };
}

#[macro_export]
macro_rules! FIELD_PREP {
    ($field:expr, $value:expr) => {
        (($value << $field.trailing_zeros()) & $field)
    };
}
