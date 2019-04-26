use breakpad_client::{register, ExceptionHandler, MinidumpDescriptor};

struct MyExceptionHandler {
    descriptor: MinidumpDescriptor,
}

impl ExceptionHandler for MyExceptionHandler {
    type Context = ();

    fn descriptor(&self) -> &MinidumpDescriptor {
        &self.descriptor
    }

    fn context(self) -> Self::Context {}
}

fn main() {
    let handler = MyExceptionHandler {
        descriptor: "/tmp".into(),
    };
    register(handler);

    unsafe {
        let ptr = std::ptr::null_mut();
        *ptr = 1;
    }
}
