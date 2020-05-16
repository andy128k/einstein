use std::cell::Cell;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use crate::error::format_err;
use crate::ui::common::Size;
use crate::ui::widget::widget::*;
use crate::ui::brick::*;
use crate::ui::context::Context;
use crate::resources::manager::ResourceManager;
use crate::resources::audio::CLICK;

pub trait ButtonRenderer {
    fn draw(&self, resource_manager: &dyn ResourceManager, highlighted: bool) -> Brick;
}

pub struct Button<R: ButtonRenderer, A> {
    size: Size,
    highlighted: Cell<bool>,
    keys: Vec<Keycode>,
    action: A,
    renderer: R,
}

impl<R: ButtonRenderer, A> Button<R, A> {
    pub fn new(size: Size, keys: &[Keycode], action: A, renderer: R) -> Button<R, A> {
        Button::<R, A> {
            size,
            highlighted: Cell::new(false),
            keys: keys.to_vec(),
            action,
            renderer,
        }
    }
}

impl<A, R: ButtonRenderer> Widget<A> for Button<R, A> where A: Clone {
    fn get_size(&self) -> Size { self.size }

    fn on_event(&mut self, event: &Event, context: &dyn Context) -> EventResult<A> {
        match *event {
            Event::KeyDown(key) if self.keys.contains(&key) => {
                context.audio().play(&*context.resource_manager().chunk(&CLICK)).map_err(|e| format_err!("{}", e))?;
                Ok(EventReaction::update_and_action(self.action.clone()))
            },
            Event::MouseButtonDown(MouseButton::Left, x, y) if self.get_size().to_rect().contains_point((x, y)) => {
                context.audio().play(&*context.resource_manager().chunk(&CLICK)).map_err(|e| format_err!("{}", e))?;
                Ok(EventReaction::update_and_action(self.action.clone()))
            },
            Event::MouseMove(x, y) => {
                let to_highlight = self.get_size().to_rect().contains_point((x, y));
                if self.highlighted.get() != to_highlight {
                    self.highlighted.set(to_highlight);
                    Ok(EventReaction::update())
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
