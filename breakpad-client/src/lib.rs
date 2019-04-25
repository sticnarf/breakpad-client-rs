use breakpad_sys::root::{register_handler_from_fd, register_handler_from_path};
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

pub struct ExceptionHandler<C> {
    descriptor: MinidumpDescriptor,
    context: Option<Box<C>>,
    filter: Option<Box<dyn FnOnce(Box<C>) -> bool>>,
    callback: Option<Box<dyn FnOnce(MinidumpDescriptor, Box<C>, bool) -> bool>>,
}

impl<C> ExceptionHandler<C> {
    pub fn new(descriptor: impl Into<MinidumpDescriptor>) -> Self {
        ExceptionHandler {
            descriptor: descriptor.into(),
            context: None,
            filter: None,
            callback: None,
        }
    }

    pub fn context(mut self, context: Box<C>) -> Self {
        self.context = Some(context);
        self
    }

    pub fn filter(mut self, filter: Box<dyn FnOnce(Box<C>) -> bool>) -> Self {
        self.filter = Some(filter);
        self
    }

    pub fn callback(
        mut self,
        callback: Box<dyn FnOnce(MinidumpDescriptor, Box<C>, bool) -> bool>,
    ) -> Self {
        self.callback = Some(callback);
        self
    }

    pub fn register(self) {
        match self.descriptor {
            MinidumpDescriptor::Directory(path) => {
                let path = path.as_os_str().as_bytes();
                let c_path = CString::new(path).expect("Path contains zero byte");
                unsafe {
                    register_handler_from_path(
                        c_path.as_ptr(),
                        None,
                        None,
                        null_mut() as *mut c_void,
                    );
                }
            }
            MinidumpDescriptor::Fd(fd) => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
