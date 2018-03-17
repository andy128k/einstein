use sdl::video::Surface;
use sdl::event::{Key, Mouse};
use sdl2::rect::{Rect};
use error::*;
use ui::widget::widget::*;

pub struct AnyKey {
    action: Box<Fn() -> Option<Effect>>
}

impl AnyKey {
    pub fn new<A: Fn() -> Option<Effect> + 'static>(action: A) -> AnyKey {
        Self {
            action: Box::new(action)
        }
    }
}

impl Widget for AnyKey {
    fn get_rect(&self) -> Rect { Rect::new(0, 0, 1, 1) }

    fn on_key_down(&self, _key: Key, _ch: u16) -> Option<Effect> {
        // sound->play(L"click.wav");
        (*self.action)()
    }

    fn on_mouse_button_down(&self, _button: Mouse, _x: u16, _y: u16) -> Option<Effect> {
        // sound->play(L"click.wav");
        (*self.action)()
    }

    fn draw(&self, _surface: &Surface) -> Result<()> {
        Ok(())
    }
}
