use breakpad_client::{register, ExceptionHandler};

struct MyExceptionHandler;

impl ExceptionHandler for MyExceptionHandler {
    type Context = ();

    fn context(self) -> Self::Context {}
}

fn main() {
    register("/tmp", MyExceptionHandler);

    unsafe {
        let ptr = std::ptr::null_mut();
        *ptr = 1;
    }
}
