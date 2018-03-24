use std::time::{Duration, Instant};
use std::rc::Rc;
use std::cell::{Cell};
use debug_cell::RefCell;
use sdl::video::Surface;
use error::*;
use storage::*;
use ui::component::game::game_run;
use ui::component::load_dialog::load_game;

#[no_mangle]
pub fn ein_game_load(surface_ptr: * mut ::sdl::video::ll::SDL_Surface, storage_ptr: *const Rc<RefCell<Storage>>) -> ::libc::c_int {
    let surface = Rc::new( ::sdl::video::Surface { raw: surface_ptr, owned: false } );
    let storage: &Rc<RefCell<Storage>> = unsafe { &* storage_ptr };

    let game_opt = load_game(surface.clone(), &storage.borrow()).unwrap();
    if let Some(game) = game_opt {
        game_run(surface.clone(), Rc::new(RefCell::new(game)), storage.clone()).unwrap() as i32
    } else {
        0
    }
}
