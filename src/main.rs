extern crate libc;
#[macro_use] extern crate failure;

mod error;

use std::ffi::CString;
use std::env::home_dir;
use std::fs::create_dir_all;
use failure::err_msg;
use error::*;

extern "C" {
    fn mainpp(argv0: *const ::libc::c_char) -> *const ::libc::c_int;
}

fn real_main() -> Result<()> {
    let home = home_dir().ok_or_else(|| err_msg("Home directory is not detected."))?;
    create_dir_all(home.join(".einstein"))?;

    unsafe {
        let argv0 = CString::new("./").unwrap();
        mainpp(argv0.as_ptr());
    }
    Ok(())
}

fn main() {
    real_main().unwrap();
}
