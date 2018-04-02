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
extern crate debug_cell;
#[cfg(windows)]
extern crate winapi;

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
use debug_cell::RefCell;
use std::ffi::{CString};
use std::mem::transmute;
use std::env::home_dir;
use std::fs::create_dir_all;
use failure::err_msg;
use sdl::sdl::{init, InitFlag, get_error, quit};
use sdl::wm::set_caption;
use sdl::event::{enable_key_repeat, RepeatDelay, RepeatInterval, enable_unicode};
use sdl::video::{set_video_mode, SurfaceFlag, VideoFlag};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::rect::Rect;
use sdl2::mixer;
use error::*;
use rules::{Possibilities, SolvedPuzzle, Thing, Rule, apply};
use puzzle_gen::generate_puzzle;
use resources::messages::get_messages;
use ui::context::Context;
use ui::component::menu::make_menu;
use ui::main_loop::main_loop;

#[no_mangle]
pub extern fn ein_generate_puzzle(sp: *mut *const SolvedPuzzle, r: *mut *const Rule, rs: *mut ::libc::size_t) {
    let (puzzle, rules) = generate_puzzle().unwrap();
    unsafe {
        *sp = transmute(Box::new(puzzle));
        *rs = rules.len();
        for (i, rule) in rules.into_iter().enumerate() {
            *r.offset(i as isize) = transmute(Box::new(rule));
        }
    }
}

#[no_mangle]
pub extern fn ein_solved_puzzle_clone(puzzle: * const SolvedPuzzle) -> * const SolvedPuzzle {
    let p: &SolvedPuzzle = unsafe { &*puzzle };
    let n = Box::new(p.clone());
    unsafe { transmute(n) }
}

#[no_mangle]
pub extern fn ein_solved_puzzle_free(puzzle: *const SolvedPuzzle) {
    let _: Box<SolvedPuzzle> = unsafe { transmute(puzzle) };
}

#[no_mangle]
pub extern fn ein_rule_is_vertical(r: * const Rule) -> ::libc::c_int {
    let rule: &Rule = unsafe { &*r };
    match *rule {
        Rule::Under(..) => 1,
        _ => 0
    }
}

#[no_mangle]
pub extern fn ein_rule_is_horizontal(r: * const Rule) -> ::libc::c_int {
    let rule: &Rule = unsafe { &*r };
    match *rule {
        Rule::Near(..) |
        Rule::Between(..) |
        Rule::Direction(..) => 1,
        _ => 0
    }
}

#[no_mangle]
pub extern fn ein_rule_free(r: * const Rule) {
    let _: Box<Rule> = unsafe { transmute(r) };
}

#[no_mangle]
pub extern fn ein_possibilities_new() -> * const Possibilities {
    let possibilities = Box::new(Possibilities::new());
    unsafe { transmute(possibilities) }
}

#[no_mangle]
pub extern fn ein_possibilities_open_initials(p: * const Possibilities, rules: * const * const Rule, count: ::libc::size_t) -> * const Possibilities {
    let mut pos: Box<Possibilities> = unsafe { transmute(p) };
    for i in 0..count {
        let rule: &Rule = unsafe { &**rules.offset(i as isize) };
        if let Rule::Open(..) = *rule {
            pos = Box::new(apply(&pos, rule));
        }
    }
    unsafe { transmute(pos) }
}

#[no_mangle]
pub extern fn ein_possibilities_is_possible(p: *const Possibilities, col: ::libc::c_int, row: ::libc::c_int, value: ::libc::c_int) -> ::libc::c_int {
    let pos: &Possibilities = unsafe { &*p };
    pos.is_possible(col as u8, Thing { row: row as u8, value: value as u8 - 1 }) as i32
}

#[no_mangle]
pub extern fn ein_possibilities_set(p: *mut Possibilities, x: ::libc::c_int, y: ::libc::c_int, v: ::libc::c_int) -> * const Possibilities {
    let pos: Box<Possibilities> = unsafe { transmute(p) };
    let new_pos = pos.set(x as u8, y as u8, v as u8 - 1);
    unsafe { transmute(Box::new(new_pos)) }
}

#[no_mangle]
pub extern fn ein_possibilities_exclude(p: *mut Possibilities, x: ::libc::c_int, y: ::libc::c_int, v: ::libc::c_int) -> * const Possibilities {
    let pos: Box<Possibilities> = unsafe { transmute(p) };
    let new_pos = pos.exclude(x as u8, y as u8, v as u8 - 1);
    unsafe { transmute(Box::new(new_pos)) }
}

#[no_mangle]
pub extern fn ein_possibilities_is_defined(p: *const Possibilities, x: ::libc::c_int, y: ::libc::c_int) -> ::libc::c_int {
    let pos: &Possibilities = unsafe { &*p };
    pos.is_defined(x as u8, y as u8) as i32
}

#[no_mangle]
pub extern fn ein_possibilities_get_defined(p: *const Possibilities, x: ::libc::c_int, y: ::libc::c_int) -> ::libc::c_int {
    let pos: &Possibilities = unsafe { &*p };
    pos.get_defined(x as u8, y as u8).unwrap_or(0u8) as i32 + 1
}

#[no_mangle]
pub extern fn ein_possibilities_is_valid(p: *const Possibilities, s: *const SolvedPuzzle) -> bool {
    let pos: &Possibilities = unsafe { &*p };
    let sp: &SolvedPuzzle = unsafe { &*s };
    pos.is_valid(sp)
}

#[no_mangle]
pub extern fn ein_possibilities_is_solved(p: * const Possibilities) -> bool {
    let pos: &Possibilities = unsafe { &*p };
    pos.is_solved()
}

#[no_mangle]
pub extern fn ein_possibilities_free(p: * const Possibilities) {
    let _: Box<Possibilities> = unsafe { transmute(p) };
}

#[no_mangle]
pub extern fn ein_get_language() -> *const ::libc::c_char {
    let language = locale::get_language().unwrap_or_default();
    let c_str = CString::new(language).unwrap();
    c_str.into_raw()
}

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

        let menu = make_menu(get_messages(), state.clone())?;
        main_loop(&context, &*menu)?;
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
