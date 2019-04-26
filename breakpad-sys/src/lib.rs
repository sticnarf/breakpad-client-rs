//! C interfaces to the breakpad library
//!
//! Google breakpad library prevents a `MinidumpDescriptor` from being returned
//! from a function. So we cannot provide a function to create a function one
//! then register it later. Instead, the library provides functions to create
//! the descriptor and register it all at once.

use std::os::raw::{c_char, c_int, c_void};

/// A C wrapper of `MinidumpDescription` class.
///
/// It will only provide read-only information you can get from the
/// minidump callbacks.
#[repr(C)]
pub struct DescriptorInfo {
    pub c_path: *const c_char,
}

type FilterCallback = Option<extern "C" fn(*mut c_void) -> bool>;
type WrappedMinidumpCallback = Option<extern "C" fn(DescriptorInfo, *mut c_void, bool) -> bool>;

extern "C" {
    /// Create a `MinidumpDescriptor` from directory path and
    /// register an `ExceptionHandler` with it.
    pub fn register_handler_from_path(
        c_path: *const c_char,
        filter: FilterCallback,
        callback: WrappedMinidumpCallback,
        callback_context: *mut c_void,
    );

    /// Create a `MinidumpDescriptor` from a file descriptor and
    /// register an `ExceptionHandler` with it.
    pub fn register_handler_from_fd(
        fd: c_int,
        filter: FilterCallback,
        callback: WrappedMinidumpCallback,
        callback_context: *mut c_void,
    );
}
