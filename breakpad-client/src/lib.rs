use breakpad_sys::{register_handler_from_fd, register_handler_from_path, DescriptorInfo};
use std::ffi::CString;
use std::os::raw::c_void;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::{FromRawFd, RawFd};
use std::path::PathBuf;
use std::ptr::null_mut;

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
    match handler.descriptor() {
        MinidumpDescriptor::Directory(path) => {
            let path = path.as_os_str().as_bytes();
            let c_path = CString::new(path).expect("Path contains zero byte");
            let ctx = Box::into_raw(Box::new(handler.context()));
            let context: Box<RustContext<H>> = Box::new(RustContext {
                filter_callback: H::filter_callback,
                minidump_callback: H::minidump_callback,
                ctx,
            });
            unsafe {
                register_handler_from_path(
                    c_path.as_ptr(),
                    Some(filter_callback_wrapper::<H>),
                    Some(minidump_callback_wrapper::<H>),
                    Box::into_raw(context) as *mut c_void,
                );
            }
        }
        MinidumpDescriptor::Fd(fd) => unsafe {
            register_handler_from_fd(
                *fd,
                Some(filter_callback_wrapper::<H>),
                Some(minidump_callback_wrapper::<H>),
                null_mut() as *mut c_void,
            );
        },
    }
}

//pub struct ExceptionHandler<C> {
//    descriptor: MinidumpDescriptor,
//    context: Option<Box<C>>,
//    filter: Option<Box<dyn FnOnce(Box<C>) -> bool>>,
//    callback: Option<Box<dyn FnOnce(MinidumpDescriptor, Box<C>, bool) -> bool>>,
//}
//
//impl<C> ExceptionHandler<C> {
//    pub fn new(descriptor: impl Into<MinidumpDescriptor>) -> Self {
//        ExceptionHandler {
//            descriptor: descriptor.into(),
//            context: None,
//            filter: None,
//            callback: None,
//        }
//    }
//
//    pub fn context(mut self, context: Box<C>) -> Self {
//        self.context = Some(context);
//        self
//    }
//
//    pub fn filter(mut self, filter: Box<dyn FnOnce(Box<C>) -> bool>) -> Self {
//        self.filter = Some(filter);
//        self
//    }
//
//    pub fn callback(
//        mut self,
//        callback: Box<dyn FnOnce(MinidumpDescriptor, Box<C>, bool) -> bool>,
//    ) -> Self {
//        self.callback = Some(callback);
//        self
//    }
//
//    pub fn register(self) {
//        match self.descriptor {
//            MinidumpDescriptor::Directory(path) => {
//                let path = path.as_os_str().as_bytes();
//                let c_path = CString::new(path).expect("Path contains zero byte");
//                unsafe {
//                    register_handler_from_path(
//                        c_path.as_ptr(),
//                        None,
//                        None,
//                        null_mut() as *mut c_void,
//                    );
//                }
//            }
//            MinidumpDescriptor::Fd(fd) => unsafe {
//                register_handler_from_fd(fd, None, None, null_mut() as *mut c_void);
//            },
//        }
//    }
//}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
