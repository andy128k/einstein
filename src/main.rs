extern crate libc;
#[macro_use] extern crate failure;
extern crate itertools;
extern crate rand;
extern crate sdl;
extern crate sdl2;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate regex;
#[macro_use] extern crate lazy_static;
#[cfg(windows)]
extern crate winapi;

pub mod cell;
pub mod error;
pub mod algebra;
pub mod util;
pub mod locale;
pub mod converge;
pub mod rules;
pub mod puzzle_gen;
pub mod resources;
pub mod ui;
pub mod storage;
pub mod audio;

use std::process::exit;
use std::rc::Rc;
use cell::RefCell;
use std::env::home_dir;
use std::fs::create_dir_all;
use failure::err_msg;
use sdl::sdl::{init, InitFlag, get_error, quit};
use sdl::wm::set_caption;
use sdl::event::{enable_key_repeat, RepeatDelay, RepeatInterval, enable_unicode};
use sdl::video::{set_video_mode, SurfaceFlag, VideoFlag};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::mixer;
use error::*;
use resources::messages::get_messages;
use ui::context::{Context, Rect};
use ui::component::menu::make_menu;
use ui::main_loop::main_loop;

pub struct AppContext {
    pub ttf: Sdl2TtfContext,
}

fn real_main() -> Result<()> {
    let home = home_dir().ok_or_else(|| err_msg("Home directory is not detected."))?;
    create_dir_all(home.join(".einstein"))?;

    let state = Rc::new(RefCell::new(storage::Storage::load().unwrap_or_default()));

    if !init(&[InitFlag::Video, InitFlag::Audio]) {
        return Err(err_msg(get_error()));
    }
    set_caption("Einstein 3.0", "");

    ensure!(enable_key_repeat(RepeatDelay::Default, RepeatInterval::Default), "Key repeat is not set.");
    enable_unicode(true);

    let app_context = AppContext {
        ttf: sdl2::ttf::init()?
    };

    unsafe {
        resources::fonts::init_fonts(::std::mem::transmute(&app_context.ttf))?;
    }

    let fullscreen = state.borrow().fullscreen;
    let volume = state.borrow().volume;

    let flags: &[VideoFlag] = if fullscreen { &[VideoFlag::Fullscreen] } else { &[] };
    let surface = Rc::new(set_video_mode(800, 600, 24, &[SurfaceFlag::SWSurface], flags).map_err(err_msg)?);

    {
        // mixer::init(mixer::InitFlag::empty()).map_err(err_msg)?;
        // let audio = audio::Audio::new()?;

        let context = Context {
            surface: surface.clone(),
            rect: Rect::new(0, 0, 800, 600)
        };

        let mut menu = make_menu(get_messages(), state.clone())?;
        main_loop(&context, &mut menu)?;
    }

    quit();

    state.borrow_mut().save()?;

    Ok(())
}

fn main() {
    if let Err(err) = real_main() {
        eprintln!("{:#?}", err);
        exit(1);
    }
}
