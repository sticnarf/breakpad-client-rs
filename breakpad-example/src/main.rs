use breakpad_client::{register, DescriptorInfo, ExceptionHandler};
use std::ffi::CStr;

struct MyExceptionHandler;

impl ExceptionHandler for MyExceptionHandler {
    type Context = ();

    fn context(self) -> Self::Context {}

    fn minidump_callback(
        descriptor: DescriptorInfo,
        _context: &'static mut Self::Context,
        succeeded: bool,
    ) -> bool {
        let path = unsafe { CStr::from_ptr(descriptor.c_path) };
        eprintln!("Dumpfile path: {}", path.to_str().unwrap());
        succeeded
    }
}

fn main() {
    register("/tmp", MyExceptionHandler);

    unsafe {
        let ptr = std::ptr::null_mut();
        *ptr = 1;
    }
}
