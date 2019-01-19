mod cell;
mod error;
mod algebra;
mod util;
mod locale;
mod converge;
mod rules;
mod puzzle_gen;
#[macro_use]
mod resources;
mod ui;
mod storage;
mod audio;

use std::process::exit;
use std::rc::Rc;
use crate::cell::RefCell;
use dirs::home_dir;
use std::fs::create_dir_all;
use failure::err_msg;
use crate::error::*;
use crate::resources::messages::get_messages;
use crate::ui::component::menu::make_menu;
use crate::ui::context::*;
use crate::audio::*;

fn real_main() -> Result<()> {
    let home = home_dir().ok_or_else(|| err_msg("Home directory is not detected."))?;
    create_dir_all(home.join(".einstein"))?;

    let state = Rc::new(RefCell::new(storage::Storage::load().unwrap_or_default()));

    let sdl_context = sdl2::init().map_err(err_msg)?;
    let video_subsystem = sdl_context.video().map_err(err_msg)?;
    let _audio_subsystem = sdl_context.audio().map_err(err_msg)?;

    let window = {
        let mut builder = video_subsystem.window("Einstein 3.0", 800, 600);
        if state.borrow().fullscreen {
            builder.fullscreen();
        }
        builder.position_centered().build()?
    };
    let mut canvas = window.into_canvas().build()?;

    let ttf = sdl2::ttf::init()?;
    let audio = SdlAudio::new().map_err(err_msg)?;

    audio.set_volume(state.borrow().volume);

    {
        let texture_creator = canvas.texture_creator();
        let mut resource_manager = resources::manager::ResourceManagerImpl::new(&texture_creator, &ttf);

        let context = crate::ui::context::AppContext {
            sdl_context: &sdl_context,
            canvas: RefCell::new(&mut canvas),
            resource_manager: &mut resource_manager,
            audio: &audio,
        };

        let mut menu = make_menu(get_messages(), state.clone())?;
        context.main_loop(&mut menu)?;
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