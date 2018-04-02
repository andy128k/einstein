use std::cell::Cell;
use sdl::event::{Key, Mouse};
use sdl2::rect::Rect;
use error::*;
use ui::context::Context;
use ui::widget::widget::*;

pub trait ButtonRenderer {
    fn draw(&self, context: &Context, highlighted: bool) -> Result<()>;
}

pub struct Button<R: ButtonRenderer, A> {
    rect: Rect,
    highlighted: Cell<bool>,
    key: Option<Key>,
    action: A,
    renderer: R,
}

impl<R: ButtonRenderer, A> Button<R, A> {
    pub fn new(rect: Rect, key: Option<Key>, action: A, renderer: R) -> Button<R, A> {
        Button::<R, A> {
            rect,
            highlighted: Cell::new(false),
            key,
            action,
            renderer,
        }
    }
}

impl<A, R: ButtonRenderer> Widget<A> for Button<R, A> where A: Clone {
    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn on_event(&self, event: &Event) -> EventReaction<A> {
        match *event {
            Event::KeyDown(key, _) if Some(key) == self.key => {
                // sound->play(L"click.wav"); TODO
                EventReaction::Action(self.action.clone())
            },
            Event::MouseButtonDown(Mouse::Left, x, y) if self.rect.contains_point((x, y)) => {
                // sound->play(L"click.wav"); TODO
                EventReaction::Action(self.action.clone())
            },
            Event::MouseMove(x, y) => {
                let to_highlight = self.rect.contains_point((x, y));
                if self.highlighted.get() != to_highlight {
                    self.highlighted.set(to_highlight);
                    EventReaction::Redraw
                } else {
                    EventReaction::NoOp
                }
            },
            _ => EventReaction::NoOp,
        }
    }

    fn draw(&self, context: &Context) -> Result<()> {
        self.renderer.draw(&context.relative(self.rect), self.highlighted.get())
    }
}
