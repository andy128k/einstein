extern crate libc;
#[macro_use] extern crate failure;
extern crate itertools;
extern crate rand;
extern crate sdl;

mod error;
mod converge;
mod rules;
mod puzzle_gen;
mod thing;
mod iconset;
mod ui;

use std::mem::transmute;
use std::env::home_dir;
use std::fs::create_dir_all;
use failure::err_msg;
use sdl::sdl::{init, InitFlag, get_error, quit};
use sdl::wm::set_caption;
use error::*;
use rules::{Possibilities, SolvedPuzzle, Thing, Rule, apply};
use puzzle_gen::generate_puzzle;

extern "C" {
    fn loadResources() -> ::libc::c_void;
    fn initAudio() -> ::libc::c_void;
    fn mainpp() -> *const ::libc::c_void;
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
    pos.is_possible(col as u8, &Thing { row: row as u8, value: value as u8 }) as i32
}

#[no_mangle]
pub extern fn ein_possibilities_set(p: *mut Possibilities, x: ::libc::c_int, y: ::libc::c_int, v: ::libc::c_int) -> * const Possibilities {
    let pos: Box<Possibilities> = unsafe { transmute(p) };
    let new_pos = pos.set(x as u8, y as u8, v as u8);
    unsafe { transmute(Box::new(new_pos)) }
}

#[no_mangle]
pub extern fn ein_possibilities_exclude(p: *mut Possibilities, x: ::libc::c_int, y: ::libc::c_int, v: ::libc::c_int) -> * const Possibilities {
    let pos: Box<Possibilities> = unsafe { transmute(p) };
    let new_pos = pos.exclude(x as u8, y as u8, v as u8);
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
    pos.get_defined(x as u8, y as u8).unwrap_or(0u8) as i32
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
