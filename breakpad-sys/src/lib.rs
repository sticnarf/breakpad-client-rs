use std::os::raw::{c_char, c_int, c_void};

#[repr(C)]
pub struct DescriptorInfo {
    c_path: *const c_char,
}

type FilterCallback = Option<extern "C" fn(*mut c_void) -> bool>;
type WrappedMinidumpCallback = Option<extern "C" fn(DescriptorInfo, *mut c_void, bool) -> bool>;

extern "C" {
    pub fn register_handler_from_path(
        c_path: *const c_char,
        filter: FilterCallback,
        callback: WrappedMinidumpCallback,
        callback_context: *mut c_void,
    );

    pub fn register_handler_from_fd(
        fd: c_int,
        filter: FilterCallback,
        callback: WrappedMinidumpCallback,
        callback_context: *mut c_void,
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
