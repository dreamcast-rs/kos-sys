// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

// KOS_INIT_FLAG_WEAK not bound:
// No stable/accepted way to create weak symbol here

#[macro_export]
macro_rules! KOS_INIT_FLAG_CALL {
    ($func:expr) => {{
        let mut ret = 0;
        if let Some(func) = $func {
            func();
            ret = 1;
        }
        ret
    }}
}

#[macro_export]
macro_rules! KOS_INIT_FLAG_ALL {
    ($flags:expr, $mask:expr, $func:ident) => {
        $crate::paste::paste! {
            extern "C" { fn $func(); }
            #[no_mangle]
            pub static [<$func _weak>]: Option<unsafe extern "C" fn()> =
                if (($flags) & $mask) == $mask { None } else { Some($func) };
        }
    };
}

#[macro_export]
macro_rules! KOS_INIT_FLAG_NONE {
    ($flags:expr, $mask:expr, $func:ident) => {
        $crate::paste::paste! {
            extern "C" { fn $func(); }
            #[no_mangle]
            pub static [<$func _weak>]: Option<unsafe extern "C" fn()> =
                if (($flags) & $mask) != 0 { None } else { Some($func) };
        }
    };
}

#[macro_export]
macro_rules! KOS_INIT_FLAG {
    ($flags:expr, $mask:expr, $func:ident) => {
        $crate::paste::paste! {
            extern "C" { fn $func(); }
            #[no_mangle]
            pub static [<$func _weak>]: Option<unsafe extern "C" fn()> =
                if (($flags) & $mask) != 0 { Some($func) } else { None };
        }
    };
}
