use std::cell::Cell;
use sdl::video::Surface;
use sdl::event::{Key, Mouse};
use sdl2::rect::{Rect, Point};
use error::*;
use ui::widget::widget::*;

pub trait ButtonRenderer {
    fn draw(&self, surface: &Surface, rect: Rect, highlighted: bool) -> Result<()>;
}

pub struct Button<R: ButtonRenderer> {
    rect: Rect,
    highlighted: Cell<bool>,
    key: Option<Key>,
    action: Box<Fn() -> Option<Effect>>,
    renderer: R,
}

impl<R: ButtonRenderer> Button<R> {
    pub fn new<A: Fn() -> Option<Effect> + 'static>(rect: Rect, key: Option<Key>, action: A, renderer: R) -> Button<R> {
        Button::<R> {
            rect,
            highlighted: Cell::new(false),
            key,
            action: Box::new(action),
            renderer,
        }
    }
}

impl<R: ButtonRenderer> Widget for Button<R> {
    fn get_rect(&self) -> Rect { self.rect }

    fn on_event(&self, event: &Event) -> Option<Effect> {
        match *event {
            Event::KeyDown(key, _) if Some(key) == self.key => {
                // sound->play(L"click.wav"); TODO
                (*self.action)()
            },
            Event::MouseButtonDown(Mouse::Left, x, y) if self.rect.contains_point((x, y)) => {
                // sound->play(L"click.wav"); TODO
                (*self.action)()
            },
            Event::MouseMove(x, y) => {
                let to_highlight = self.rect.contains_point((x, y));
                if self.highlighted.get() != to_highlight {
                    self.highlighted.set(to_highlight);
                    Some(Effect::Redraw(vec![self.rect]))
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    fn draw(&self, surface: &Surface) -> Result<()> {
        self.renderer.draw(surface, self.get_rect(), self.highlighted.get())
    }
}
