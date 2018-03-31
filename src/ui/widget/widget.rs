use sdl::video::Surface;
use sdl::event::{Key, Mouse};
use sdl2::rect::Rect;
use error::*;

pub enum Event {
    Tick,
    MouseButtonDown(Mouse, i32, i32),
    MouseButtonUp(Mouse, i32, i32),
    MouseMove(i32, i32),
    KeyDown(Key, u16),
}

#[derive(Debug)]
pub enum Effect {
    Terminate,
    Quit,
    Redraw(Vec<Rect>),
    NoOp
}

pub trait Widget {
    fn get_rect(&self) -> Rect;
    fn on_event(&self, _event: &Event) -> Option<Effect> { None }
    fn draw(&self, surface: &Surface) -> Result<()>;
    fn draw_in_rects(&self, surface: &Surface, rects: &[Rect]) -> Result<()> {
        let this_rect = self.get_rect();
        if rects.iter().any(|rect| this_rect.has_intersection(*rect)) {
            self.draw(surface)
        } else {
            Ok(())
        }
    }
}
