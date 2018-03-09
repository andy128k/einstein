extern crate libc;
#[macro_use] extern crate failure;
extern crate itertools;
extern crate rand;
extern crate sdl;
extern crate sdl2;
extern crate sdl2_ttf;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate toml;
extern crate regex;
#[macro_use] extern crate lazy_static;

pub mod error;
pub mod util;
pub mod locale;
pub mod converge;
pub mod rules;
pub mod puzzle_gen;
pub mod ui;
pub mod storage;
pub mod text_parser;

use std::ffi::{CStr, CString};
use std::ptr::null;
use std::mem::transmute;
use std::env::home_dir;
use std::fs::create_dir_all;
use failure::err_msg;
use sdl::sdl::{init, InitFlag, get_error, quit};
use sdl::wm::set_caption;
use sdl2_ttf::Sdl2TtfContext;
use error::*;
use rules::{Possibilities, SolvedPuzzle, Thing, Rule, apply};
use puzzle_gen::generate_puzzle;

extern "C" {
    fn loadResources() -> ::libc::c_void;
    fn initAudio(volume: ::libc::c_int) -> ::libc::c_void;
    fn mainpp(fullscreen: ::libc::c_int, config: *const ::libc::c_void, top_scores: *const ::libc::c_void) -> *const ::libc::c_void;
}

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
pub extern fn ein_rule_draw(r: * const Rule, surface_ptr: * mut sdl::video::ll::SDL_Surface, x: ::libc::c_int, y: ::libc::c_int, h: ::libc::c_int) {
    let rule: &Rule = unsafe { &*r };
    let surface = sdl::video::Surface { raw: surface_ptr, owned: false };
    ui::rule::draw_rule(rule, &surface, x as i16, y as i16, h != 0);
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
    pos.is_possible(col as u8, &Thing { row: row as u8, value: value as u8 - 1 }) as i32
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
pub extern fn ein_draw_thing(t: ::libc::c_int, v: ::libc::c_int, surface_ptr: * mut sdl::video::ll::SDL_Surface, x: ::libc::c_int, y: ::libc::c_int, h: ::libc::c_int) {
    let surface = sdl::video::Surface { raw: surface_ptr, owned: false };
    ui::rule::draw_thing(&Thing { row: t as u8, value: (v - 1) as u8 }, &surface, x as i16, y as i16, h != 0);
}

#[no_mangle]
pub extern fn ein_draw_small_thing(t: ::libc::c_int, v: ::libc::c_int, surface_ptr: * mut sdl::video::ll::SDL_Surface, x: ::libc::c_int, y: ::libc::c_int, h: ::libc::c_int) {
    let surface = sdl::video::Surface { raw: surface_ptr, owned: false };
    ui::rule::draw_small_thing(&Thing { row: t as u8, value: (v - 1) as u8 }, &surface, x as i16, y as i16, h != 0);
}

#[no_mangle]
pub extern fn ein_config_get_last_name(c: *const ::libc::c_void) -> * const ::libc::c_char {
    let s: &storage::Storage = unsafe { &* (c as *const storage::Storage) };
    match s.last_name {
        Some(ref n) => {
            let nn = n.clone();
            let c_str = CString::new(nn).unwrap();
            c_str.into_raw()
        },
        _ => null()
    }
}

#[no_mangle]
pub extern fn ein_config_set_last_name(c: *mut ::libc::c_void, l: * const ::libc::c_char) {
    let s: &mut storage::Storage = unsafe { &mut * (c as *mut storage::Storage) };
    s.last_name = if l.is_null() {
        None
    } else {
        unsafe {
            let cstr = CStr::from_ptr(l);
            Some(cstr.to_str().unwrap().to_owned())
        }
    };
}

#[no_mangle]
pub extern fn ein_config_get_fullscreen(c: *const ::libc::c_void) -> ::libc::c_int {
    let s: &storage::Storage = unsafe { &* (c as *const storage::Storage) };
    s.fullscreen as ::libc::c_int
}

#[no_mangle]
pub extern fn ein_config_set_fullscreen(c: *mut ::libc::c_void, l: ::libc::c_int) {
    let s: &mut storage::Storage = unsafe { &mut * (c as *mut storage::Storage) };
    s.fullscreen = l != 0;
}

#[no_mangle]
pub extern fn ein_config_get_volume(c: *const ::libc::c_void) -> ::libc::c_int {
    let s: &storage::Storage = unsafe { &* (c as *const storage::Storage) };
    s.volume as ::libc::c_int
}

#[no_mangle]
pub extern fn ein_config_set_volume(c: *mut ::libc::c_void, l: ::libc::c_int) {
    let s: &mut storage::Storage = unsafe { &mut * (c as *mut storage::Storage) };
    s.volume = l as u32;
}

#[no_mangle]
pub extern fn ein_topscores_is_deserving(c: *const ::libc::c_void, ch: ::libc::c_int) -> ::libc::c_int {
    let s: &storage::Scores = unsafe { &* (c as *const storage::Scores) };
    s.is_deserving(ch as u32) as ::libc::c_int
}

#[no_mangle]
pub extern fn ein_topscores_add(c: *mut ::libc::c_void, l: * const ::libc::c_char, ch: ::libc::c_int) -> ::libc::c_int {
    let s: &mut storage::Scores = unsafe { &mut * (c as *mut storage::Scores) };
    let name: String = if l.is_null() {
        String::new()
    } else {
        unsafe {
            let cstr = CStr::from_ptr(l);
            cstr.to_str().unwrap().to_owned()
        }
    };
    s.add_score_entry(storage::Score { name, score: ch as u32 }).map(|c| c as i32).unwrap_or(-1)
}

#[no_mangle]
pub extern fn ein_topscores_get_count(c: *const ::libc::c_void) -> ::libc::c_int {
    let s: &storage::Scores = unsafe { &* (c as *const storage::Scores) };
    s.0.len() as ::libc::c_int
}

#[no_mangle]
pub extern fn ein_topscores_get_name(c: *const ::libc::c_void, ch: ::libc::c_int) -> *const ::libc::c_char {
    let s: &storage::Scores = unsafe { &* (c as *const storage::Scores) };
    let nn = &s.0[ch as usize].name;
    let c_str = CString::new(nn.clone()).unwrap();
    c_str.into_raw()
}

#[no_mangle]
pub extern fn ein_topscores_get_score(c: *const ::libc::c_void, ch: ::libc::c_int) -> ::libc::c_int {
    let s: &storage::Scores = unsafe { &* (c as *const storage::Scores) };
    s.0[ch as usize].score as ::libc::c_int
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

    let mut state = storage::Storage::load().unwrap_or_default();

    unsafe {
        loadResources();
    }

    if !init(&[InitFlag::Video, InitFlag::Audio]) {
        return Err(err_msg(get_error()));
    }
    set_caption("Einstein 3.0", "");

    let app_context = AppContext {
        ttf: sdl2_ttf::init()?
    };

    ui::fonts::init_fonts(&app_context.ttf)?;

    unsafe {
        initAudio(state.volume as i32);
        mainpp(state.fullscreen as i32, transmute(&state), transmute(&state.scores));
    }

    quit();

    state.save()?;

    Ok(())
}

fn main() {
    real_main().unwrap();
}
