extern crate libc;
#[macro_use] extern crate failure;
extern crate itertools;
extern crate rand;
extern crate sdl2;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate regex;
#[macro_use] extern crate lazy_static;
#[cfg(windows)]
extern crate winapi;
extern crate dirs;

pub mod cell;
pub mod error;
pub mod algebra;
pub mod util;
pub mod locale;
pub mod converge;
pub mod rules;
pub mod puzzle_gen;
#[macro_use]
pub mod resources;
pub mod ui;
pub mod storage;
pub mod audio;

use std::process::exit;
use std::rc::Rc;
use cell::RefCell;
use dirs::home_dir;
use std::fs::create_dir_all;
use failure::err_msg;
use sdl2::{init};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::mixer;
use error::*;
use resources::messages::get_messages;
use ui::context::{Rect};
use ui::component::menu::make_menu;
use ui::main_loop::main_loop;

fn real_main() -> Result<()> {
    let home = home_dir().ok_or_else(|| err_msg("Home directory is not detected."))?;
    create_dir_all(home.join(".einstein"))?;

    let state = Rc::new(RefCell::new(storage::Storage::load().unwrap_or_default()));

    let sdl_context = sdl2::init().map_err(err_msg)?;
    let video = sdl_context.video().map_err(err_msg)?;
    let audio = sdl_context.audio().map_err(err_msg)?;

    let window = {
        let mut builder = video.window("Einstein 3.0", 800, 600);
        if state.borrow().fullscreen {
            builder.fullscreen();
        }
        builder.position_centered().build()?
    };
    let mut canvas = window.into_canvas().build()?;

    // ensure!(enable_key_repeat(RepeatDelay::Default, RepeatInterval::Default), "Key repeat is not set.");
    // enable_unicode(true);

    let ttf = sdl2::ttf::init()?;
    let audio = audio::Audio::new().map_err(err_msg)?;

    audio.set_volume(state.borrow().volume as f32 / 100.0);

    {
        let texture_creator = canvas.texture_creator();
        let mut resource_manager = resources::manager::ResourceManagerImpl::new(&texture_creator, &ttf);

        let mut menu = make_menu(get_messages(), state.clone())?;
        main_loop(&sdl_context, &mut canvas, &mut menu, &mut resource_manager)?;
    }

    state.borrow_mut().save()?;

    Ok(())
}

fn main() {
    if let Err(err) = real_main() {
        eprintln!("{:#?}", err);
        exit(1);
    }
}
