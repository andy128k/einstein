use sdl::video::Surface;
use sdl::event::{Key, Mouse};
use sdl2::rect::Rect;
use error::*;

pub enum Effect {
    Terminate,
    Redraw(Vec<Rect>),
    NoOp
}

pub trait Widget {
    fn get_rect(&self) -> Rect;
    fn on_tick(&self) -> Option<Effect> { None }
    fn on_mouse_button_down(&self, _button: Mouse, _x: u16, _y: u16) -> Option<Effect> { None }
    fn on_mouse_button_up(&self, _button: Mouse, _x: u16, _y: u16) -> Option<Effect> { None }
    fn on_mouse_move(&self, _x: u16, _y: u16) -> Option<Effect> { None }
    fn on_key_down(&self, _key: Key, _ch: u16) -> Option<Effect> { None }
    fn draw(&self, surface: &Surface) -> Result<()>;
}
