use breakpad_sys::{register_handler_from_fd, register_handler_from_path, DescriptorInfo};
use std::ffi::CStr;
use std::os::raw::c_void;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::{FromRawFd, RawFd};
use std::path::PathBuf;

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

pub trait ExceptionHandler {
    type Context: 'static;

    fn descriptor(&self) -> &MinidumpDescriptor;

    fn context(self) -> Self::Context;

    fn filter_callback(_context: &'static mut Self::Context) -> bool {
        true
    }

    fn minidump_callback(
        _descriptor: DescriptorInfo,
        _context: &'static mut Self::Context,
        succeeded: bool,
    ) -> bool {
        succeeded
    }
}

struct RustContext<H: ExceptionHandler> {
    filter_callback: fn(&'static mut H::Context) -> bool,
    minidump_callback: fn(DescriptorInfo, &'static mut H::Context, bool) -> bool,
    ctx: *mut H::Context,
}

extern "C" fn filter_callback_wrapper<H: ExceptionHandler>(ctx: *mut c_void) -> bool {
    let context = unsafe { Box::from_raw(ctx as *mut RustContext<H>) };
    let ctx = unsafe { Box::from_raw(context.ctx) };
    (context.filter_callback)(Box::leak(ctx))
}

extern "C" fn minidump_callback_wrapper<H: ExceptionHandler>(
    info: DescriptorInfo,
    ctx: *mut c_void,
    succeeded: bool,
) -> bool {
    let context = unsafe { Box::from_raw(ctx as *mut RustContext<H>) };
    let ctx = unsafe { Box::from_raw(context.ctx) };
    (context.minidump_callback)(info, Box::leak(ctx), succeeded)
}

pub fn register<H: ExceptionHandler>(handler: H) {
    macro_rules! rust_context {
        () => {{
            let ctx = Box::into_raw(Box::new(handler.context()));
            let context: Box<RustContext<H>> = Box::new(RustContext {
                filter_callback: H::filter_callback,
                minidump_callback: H::minidump_callback,
                ctx,
            });
            Box::into_raw(context) as *mut c_void
        }};
    }

    match handler.descriptor() {
        MinidumpDescriptor::Directory(path) => {
            let c_path = CStr::from_bytes_with_nul(path.as_os_str().as_bytes())
                .expect("Path is not null terminated")
                .as_ptr();
            unsafe {
                register_handler_from_path(
                    c_path,
                    Some(filter_callback_wrapper::<H>),
                    Some(minidump_callback_wrapper::<H>),
                    rust_context!(),
                );
            }
        }
        MinidumpDescriptor::Fd(fd) => unsafe {
            register_handler_from_fd(
                *fd,
                Some(filter_callback_wrapper::<H>),
                Some(minidump_callback_wrapper::<H>),
                rust_context!(),
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
