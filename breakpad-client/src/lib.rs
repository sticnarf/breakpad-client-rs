//! A Rust library to use Google breakpad client.

use breakpad_sys::{register_handler_from_fd, register_handler_from_path};
use std::ffi::CStr;
use std::os::raw::c_void;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::{FromRawFd, RawFd};
use std::path::PathBuf;

pub use breakpad_sys::DescriptorInfo;

/// Describes where to save the minidump
///
/// A directory or an opened file descriptor is supported
pub enum MinidumpDescriptor {
    Directory(PathBuf),
    Fd(RawFd),
}

impl<P: Into<PathBuf>> From<P> for MinidumpDescriptor {
    fn from(path: P) -> Self {
        MinidumpDescriptor::Directory(path.into())
    }
}

impl FromRawFd for MinidumpDescriptor {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        MinidumpDescriptor::Fd(fd)
    }
}

/// Defines the behavior when the program crashes
///
/// **Note: You should do as little work as possible in the callback function.
/// Your application is in an unsafe state. It may not be safe to allocate memory
/// or call functions from other shared libraries. The safest thing to do is fork
/// and exec a new process to do any work you need to do.**
pub trait ExceptionHandler {
    type Context: 'static;

    /// Returns the context you want to register.
    /// This context will be boxed to the heap and registered.
    fn context(self) -> Self::Context;

    /// A callback function to run before Breakpad performs any substantial
    /// processing of an exception. It is called before writing
    /// a minidump. `context` is the object supplied by the `context` function.
    ///
    /// If it returns true, Breakpad will continue processing,
    /// attempting to write a minidump.  If it returns false,
    /// Breakpad  will immediately report the exception as unhandled without
    /// writing a minidump, allowing another handler the opportunity to handle it.
    ///
    /// The default implementation just returns `true`.
    #[allow(unused_variables)]
    fn filter_callback(context: &'static mut Self::Context) -> bool {
        true
    }

    /// A callback function to run after the minidump has been written.
    /// `descriptor` contains the file descriptor or file path containing the
    /// minidump. `context` is the object supplied by the `context` function.
    /// `succeeded` indicates whether a minidump file was successfully written.
    ///
    /// If an exception occurred and the callback returns true, Breakpad will
    /// treat the exception as fully-handled, suppressing any other handlers from
    /// being notified of the exception.  If the callback returns false, Breakpad
    /// will treat the exception as unhandled, and allow another handler to handle
    /// it. If there are no other handlers, Breakpad will report the exception to
    /// the system as unhandled, allowing a debugger or native crash dialog the
    /// opportunity to handle the exception.  Most callback implementations
    /// should normally return the value of `succeeded`, or when they wish to
    /// not report an exception of handled, false.  Callbacks will rarely want to
    /// return true directly (unless `succeeded` is true).
    ///
    /// The default implementation just returns `succeeded`.
    #[allow(unused_variables)]
    fn minidump_callback(
        descriptor: DescriptorInfo,
        context: &'static mut Self::Context,
        succeeded: bool,
    ) -> bool {
        succeeded
    }
}

extern "C" fn filter_callback_wrapper<H: ExceptionHandler>(ctx: *mut c_void) -> bool {
    let context = unsafe { Box::from_raw(ctx as *mut H::Context) };
    H::filter_callback(Box::leak(context))
}

extern "C" fn minidump_callback_wrapper<H: ExceptionHandler>(
    info: DescriptorInfo,
    ctx: *mut c_void,
    succeeded: bool,
) -> bool {
    let context = unsafe { Box::from_raw(ctx as *mut H::Context) };
    H::minidump_callback(info, Box::leak(context), succeeded)
}

/// Register an `ExceptionHandler` with the given descriptor.
pub fn register<H: ExceptionHandler>(descriptor: impl Into<MinidumpDescriptor>, handler: H) {
    let ctx = Box::into_raw(Box::new(handler.context()));
    match descriptor.into() {
        MinidumpDescriptor::Directory(path) => {
            // Path on Linux should be nul terminated
            let c_path =
                unsafe { CStr::from_bytes_with_nul_unchecked(path.as_os_str().as_bytes()) };
            unsafe {
                register_handler_from_path(
                    c_path.as_ptr(),
                    Some(filter_callback_wrapper::<H>),
                    Some(minidump_callback_wrapper::<H>),
                    ctx as *mut c_void,
                );
            }
        }
        MinidumpDescriptor::Fd(fd) => unsafe {
            register_handler_from_fd(
                fd,
                Some(filter_callback_wrapper::<H>),
                Some(minidump_callback_wrapper::<H>),
                ctx as *mut c_void,
            );
        },
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
