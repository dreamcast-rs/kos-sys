// Rust for KallistiOS/Dreamcast
// Copyright (C) 2025 Eric Fradella
// https://dreamcast.rs/

use crate::prelude::*;

use super::pthreadtypes::*;

pub const PTHREAD_PROCESS_PRIVATE: c_int        = 0;
pub const PTHREAD_PROCESS_SHARED: c_int         = 1;

pub const PTHREAD_SCOPE_PROCESS: c_int          = 0;
pub const PTHREAD_SCOPE_SYSTEM: c_int           = 1;

pub const PTHREAD_CANCEL_DISABLE: c_int         = 0;
pub const PTHREAD_CANCEL_ENABLE: c_int          = 1;

pub const PTHREAD_CANCEL_DEFERRED: c_int        = 0;
pub const PTHREAD_CANCEL_ASYNCHRONOUS: c_int    = 1;

pub const PTHREAD_CREATE_DETACHED: c_int        = 0;
pub const PTHREAD_CREATE_JOINABLE: c_int        = 1;

pub const PTHREAD_STACK_MIN: c_int              = 256;
pub const PTHREAD_STACK_MIN_ALIGNMENT: c_int    = 32;

pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    __data: [0; __PTHREAD_COND_SIZE],
};

pub type pthread_key_t = c_int;

pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
    __data: [0; __PTHREAD_MUTEX_SIZE],
};

pub const PTHREAD_MUTEX_NORMAL: c_int           = 0;
pub const PTHREAD_MUTEX_DEFAULT: c_int          = PTHREAD_MUTEX_NORMAL;
pub const PTHREAD_MUTEX_ERRORCHECK: c_int       = 2;
pub const PTHREAD_MUTEX_RECURSIVE: c_int        = 3;

pub const PTHREAD_MUTEX_ROBUST: c_int           = 0;
pub const PTHREAD_MUTEX_STALLED: c_int          = 1;

pub type pthread_once_t = c_int; // volatile

pub const PTHREAD_ONCE_INIT: pthread_once_t     = 0;

pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
    __data: [0; __PTHREAD_RWLOCK_SIZE],
};

pub type pthread_spinlock_t = c_int; // volatile

pub const PTHREAD_BARRIER_SERIAL_THREAD: c_int  = 0x7fffffff;

#[link(name = "kallisti")]
unsafe extern "C" {
    pub fn pthread_create(
        thread: *mut pthread_t,
        attr: *const pthread_attr_t,
        start_routine: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
        arg: *mut c_void,
    ) -> c_int;
    pub fn pthread_detach(thread: pthread_t) -> c_int;
    pub fn pthread_exit(value_ptr: *mut c_void);
    pub fn pthread_join(thread: pthread_t, value_ptr: *mut *mut c_void) -> c_int;
    pub fn pthread_self() -> pthread_t;
    pub fn pthread_setschedprio(thread: pthread_t, prio: c_int) -> c_int;
    pub fn pthread_getname_np(thread: pthread_t, buf: *mut c_char, buflen: c_size_t) -> c_int;
    pub fn pthread_setname_np(thread: pthread_t, buf: *const c_char) -> c_int;
    pub fn pthread_getprio(thread: pthread_t) -> c_int;
    pub fn pthread_setprio(thread: pthread_t, prio: c_int) -> c_int;
    pub fn pthread_attr_init(attr: *mut pthread_attr_t) -> c_int;
    pub fn pthread_attr_destroy(attr: *mut pthread_attr_t) -> c_int;
    pub fn pthread_attr_getdetachstate(
        attr: *const pthread_attr_t,
        detachstate: *mut c_int,
    ) -> c_int;
    pub fn pthread_attr_setdetachstate(attr: *mut pthread_attr_t, detachstate: c_int) -> c_int;
    pub fn pthread_attr_getguardsize(
        attr: *const pthread_attr_t,
        guardsize: *mut c_size_t,
    ) -> c_int;
    pub fn pthread_attr_setguardsize(attr: *mut pthread_attr_t, guardsize: c_size_t) -> c_int;
    pub fn pthread_attr_getschedparam(
        attr: *const pthread_attr_t,
        param: *mut sched_param,
    ) -> c_int;
    pub fn pthread_attr_setschedparam(attr: *mut pthread_attr_t, par: *const sched_param) -> c_int;
    pub fn pthread_attr_getstack(
        attr: *const pthread_attr_t,
        stackaddr: *mut *mut c_void,
        stacksize: *mut c_size_t,
    ) -> c_int;
    pub fn pthread_attr_setstack(
        attr: *mut pthread_attr_t,
        stackaddr: *mut c_void,
        stacksize: c_size_t,
    ) -> c_int;
    pub fn pthread_attr_getstacksize(
        attr: *const pthread_attr_t,
        stacksize: *mut c_size_t,
    ) -> c_int;
    pub fn pthread_attr_setstacksize(attr: *mut pthread_attr_t, stacksize: c_size_t) -> c_int;
    pub fn pthread_attr_getscope(attr: *const pthread_attr_t, contentionscope: *mut c_int)
        -> c_int;
    pub fn pthread_attr_setscope(attr: *mut pthread_attr_t, contentionscope: c_int) -> c_int;
    pub fn pthread_attr_getname_np(
        attr: *const pthread_attr_t,
        buf: *mut c_char,
        buflen: c_size_t,
    ) -> c_int;
    pub fn pthread_attr_setname_np(attr: *mut pthread_attr_t, name: *const c_char) -> c_int;
    pub fn pthread_cancel(thd: pthread_t) -> c_int;
    pub fn pthread_testcancel();
    pub fn pthread_setcancelstate(state: c_int, oldstate: *mut c_int) -> c_int;
    pub fn pthread_setcanceltype(r#type: c_int, oldtype: *mut c_int) -> c_int;
    pub fn pthread_cond_init(cond: *mut pthread_cond_t, attr: *const pthread_condattr_t) -> c_int;
    pub fn pthread_cond_destroy(cond: *mut pthread_cond_t) -> c_int;
    pub fn pthread_cond_broadcast(cond: *mut pthread_cond_t) -> c_int;
    pub fn pthread_cond_signal(cond: *mut pthread_cond_t) -> c_int;
    pub fn pthread_cond_wait(cond: *mut pthread_cond_t, mutex: *mut pthread_cond_t) -> c_int;
    pub fn pthread_cond_timedwait(
        cond: *mut pthread_cond_t,
        mutex: *mut pthread_mutex_t,
        abstime: *const timespec,
    ) -> c_int;
    pub fn pthread_condattr_init(attr: *mut pthread_condattr_t) -> c_int;
    pub fn pthread_condattr_destroy(attr: *mut pthread_condattr_t) -> c_int;
    pub fn pthread_condattr_getclock(
        attr: *const pthread_condattr_t,
        clock_id: *mut clockid_t,
    ) -> c_int;
    pub fn pthread_condattr_setclock(attr: *mut pthread_condattr_t, clock_id: clockid_t) -> c_int;
    pub fn pthread_key_create(
        key: *mut pthread_key_t,
        destructor: Option<unsafe extern "C" fn(*mut c_void)>,
    ) -> c_int;
    pub fn pthread_key_delete(key: pthread_key_t) -> c_int;
    pub fn pthread_getspecific(key: pthread_key_t) -> *mut c_void;
    pub fn pthread_setspecific(key: pthread_key_t, value: *const c_void) -> c_int;
    pub fn pthread_mutex_init(
        mutex: *mut pthread_mutex_t,
        attr: *const pthread_mutexattr_t,
    ) -> c_int;
    pub fn pthread_mutex_destroy(mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_trylock(mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_timedlock(mutex: *mut pthread_mutex_t, abstime: *const timespec) -> c_int;
    pub fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_consistent(mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutexattr_init(attr: *mut pthread_mutexattr_t) -> c_int;
    pub fn pthread_mutexattr_destroy(attr: *mut pthread_mutexattr_t) -> c_int;
    pub fn pthread_mutexattr_getrobust(at: *const pthread_mutexattr_t, robust: *mut c_int)
        -> c_int;
    pub fn pthread_mutexattr_setrobust(attr: *mut pthread_mutexattr_t, robust: c_int) -> c_int;
    pub fn pthread_mutexattr_gettype(attr: *const pthread_mutexattr_t, r#type: *mut c_int)
        -> c_int;
    pub fn pthread_mutexattr_settype(attr: *mut pthread_mutexattr_t, r#type: c_int) -> c_int;
    pub fn pthread_once(
        once_control: *mut pthread_once_t,
        init_routine: Option<unsafe extern "C" fn()>,
    ) -> c_int;
    pub fn pthread_rwlock_init(
        rwlock: *mut pthread_rwlock_t,
        attr: *const pthread_rwlockattr_t,
    ) -> c_int;
    pub fn pthread_rwlock_destroy(rwlock: *mut pthread_rwlock_t) -> c_int;
    pub fn pthread_rwlock_rdlock(rwlock: *mut pthread_rwlock_t) -> c_int;
    pub fn pthread_rwlock_timedrdlock(
        rwlock: *mut pthread_rwlock_t,
        abstm: *const timespec,
    ) -> c_int;
    pub fn pthread_rwlock_tryrdlock(rwlock: *mut pthread_rwlock_t) -> c_int;
    pub fn pthread_rwlock_wrlock(rwlock: *mut pthread_rwlock_t) -> c_int;
    pub fn pthread_rwlock_timedwrlock(
        rwlock: *mut pthread_rwlock_t,
        abstm: *const timespec,
    ) -> c_int;
    pub fn pthread_rwlock_trywrlock(rwlock: *mut pthread_rwlock_t) -> c_int;
    pub fn pthread_rwlock_unlock(rwlock: *mut pthread_rwlock_t) -> c_int;
    pub fn pthread_rwlockattr_init(attr: *mut pthread_rwlockattr_t) -> c_int;
    pub fn pthread_rwlockattr_destroy(attr: *mut pthread_rwlockattr_t) -> c_int;
    pub fn pthread_spin_init(lock: *mut pthread_spinlock_t, pshared: c_int) -> c_int;
    pub fn pthread_spin_destroy(lock: *mut pthread_spinlock_t) -> c_int;
    pub fn pthread_spin_lock(lock: *mut pthread_spinlock_t) -> c_int;
    pub fn pthread_spin_trylock(lock: *mut pthread_spinlock_t) -> c_int;
    pub fn pthread_spin_unlock(lock: *mut pthread_spinlock_t) -> c_int;
    pub fn pthread_barrier_init(
        barrier: *mut pthread_barrier_t,
        attr: *const pthread_barrierattr_t,
        count: c_uint,
    ) -> c_int;
    pub fn pthread_barrier_destroy(barrier: *mut pthread_barrier_t) -> c_int;
    pub fn pthread_barrier_wait(barrier: *mut pthread_barrier_t) -> c_int;
    pub fn pthread_barrierattr_init(attr: *mut pthread_barrierattr_t) -> c_int;
    pub fn pthread_barrierattr_destroy(attr: *mut pthread_barrierattr_t) -> c_int;
    pub fn pthread_getconcurrency() -> c_int;
    pub fn pthread_setconcurrency(new_level: c_int) -> c_int;
    pub fn pthread_atfork(
        prepare: Option<unsafe extern "C" fn()>,
        parent: Option<unsafe extern "C" fn()>,
        child: Option<unsafe extern "C" fn()>,
    ) -> c_int;
    pub fn pthread_yield() -> c_int;
}
