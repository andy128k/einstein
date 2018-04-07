use std::cell::Cell;
use sdl::event::{Key, Mouse};
use error::*;
use ui::context::{Context, Rect};
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
    fn is_relative(&self) -> bool { true }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn on_event(&mut self, event: &Event) -> EventResult<A> {
        match *event {
            Event::KeyDown(key, _) if Some(key) == self.key => {
                // sound->play(L"click.wav"); TODO
                Ok(EventReaction::update_and_action(self.get_rect(), self.action.clone()))
            },
            Event::MouseButtonDown(Mouse::Left, x, y) if self.get_client_rect().contains_point((x, y)) => {
                // sound->play(L"click.wav"); TODO
                Ok(EventReaction::update_and_action(self.get_rect(), self.action.clone()))
            },
            Event::MouseMove(x, y) => {
                let to_highlight = self.get_client_rect().contains_point((x, y));
                if self.highlighted.get() != to_highlight {
                    self.highlighted.set(to_highlight);
                    Ok(EventReaction::update(self.get_rect()))
                } else {
                    Ok(EventReaction::empty())
                }
            },
            _ => Ok(EventReaction::empty()),
        }
    }

    fn draw(&self, context: &Context) -> Result<()> {
        self.renderer.draw(context, self.highlighted.get())
    }
}
