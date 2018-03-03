extern crate libc;
use std::ffi::CString;

extern "C" {
    fn mainpp(argv0: *const ::libc::c_char) -> *const ::libc::c_int;
}

fn main() {
    unsafe {
        let argv0 = CString::new("./").unwrap();
        mainpp(argv0.as_ptr());
    }
}
