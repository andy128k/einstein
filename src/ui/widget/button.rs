use std::cell::Cell;
use sdl::event::{Key, Mouse};
use ui::context::Rect;
use ui::widget::widget::*;
use ui::widget::brick::*;
use resources::manager::ResourceManager;

pub trait ButtonRenderer {
    fn draw(&self, resource_manager: &dyn ResourceManager, highlighted: bool) -> Brick;
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

    fn draw(&self, resource_manager: &dyn ResourceManager) -> Brick {
        self.renderer.draw(resource_manager, self.highlighted.get())
    }
}
