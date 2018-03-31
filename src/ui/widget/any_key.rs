use sdl::video::Surface;
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

    fn on_event(&self, event: &Event) -> Option<Effect> {
        match *event {
            Event::KeyDown(..) | Event::MouseButtonDown(..) => {
                // sound->play(L"click.wav");
                (*self.action)()
            },
            _ => None
        }
    }

    fn draw(&self, _surface: &Surface) -> Result<()> {
        Ok(())
    }
}
