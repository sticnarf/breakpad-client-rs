use breakpad_client::ExceptionHandler;

fn main() {
    ExceptionHandler::<()>::new("/tmp").register();
    unsafe {
        let ptr = std::ptr::null_mut();
        *ptr = 1;
    }
}
