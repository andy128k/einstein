use sdl2::Sdl;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::resources::manager::ResourceManager;
use crate::audio::Audio;

pub struct Context<'c> {
    pub sdl_context: &'c Sdl,
    pub canvas: &'c mut Canvas<Window>,
    pub resource_manager: &'c dyn ResourceManager,
    pub audio: &'c dyn Audio,
}
