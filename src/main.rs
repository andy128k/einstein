extern crate libc;
#[macro_use] extern crate failure;
extern crate itertools;
extern crate rand;
extern crate sdl;

mod error;
mod converge;
mod rules;
mod puzzle_gen;

use std::env::home_dir;
use std::fs::create_dir_all;
use failure::err_msg;
use sdl::sdl::{init, InitFlag, get_error, quit};
use sdl::wm::set_caption;
use error::*;

extern "C" {
    fn loadResources() -> ::libc::c_void;
    fn initAudio() -> ::libc::c_void;
    fn mainpp() -> *const ::libc::c_void;
}

fn real_main() -> Result<()> {
    let home = home_dir().ok_or_else(|| err_msg("Home directory is not detected."))?;
    create_dir_all(home.join(".einstein"))?;

    unsafe {
        loadResources();
    }

    if !init(&[InitFlag::Video, InitFlag::Audio]) {
        return Err(err_msg(get_error()));
    }
    set_caption("Einstein 3.0", "");

    unsafe {
        initAudio();
        mainpp();
    }

    quit();

    Ok(())
}

fn main() {
    real_main().unwrap();
}
