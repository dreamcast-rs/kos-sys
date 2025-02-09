// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024, 2025 Eric Fradella
// https://dreamcast.rs/

pub mod blockdev;
pub mod cond;
pub mod dbgio;
pub mod dbglog;
pub mod dirent;
pub mod elf;
pub mod exports;
pub mod fs;
pub mod fs_dev;
pub mod fs_null;
pub mod fs_pty;
pub mod fs_ramdisk;
pub mod fs_random;
pub mod fs_romdisk;
pub mod fs_socket;
pub mod genwait;
pub mod inet;
pub mod init;
pub mod init_base;
pub mod libgen;
pub mod library;
pub mod limits;
pub mod malloc;
pub mod mutex;
pub mod net;
pub mod netdb;
pub mod netinet;
pub mod nmmgr;
pub mod once;
pub mod oneshot_timer;
pub mod opts;
pub mod poll;
pub mod pthread;
pub mod pthreadtypes;
pub mod regfield;
pub mod rwsem;
pub mod sched_param;
pub mod select;
pub mod sem;
pub mod socket;
pub mod stdlib;
pub mod thread;
pub mod threads;
pub mod types;
pub mod tls;
pub mod uio;
pub mod utime;
pub mod utsname;
pub mod version;
pub mod worker_thread;
