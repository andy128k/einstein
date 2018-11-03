use std::cell::Cell;
use failure::err_msg;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use ui::context::Rect;
use ui::widget::widget::*;
use ui::widget::brick::*;
use resources::manager::ResourceManager;
use audio::Audio;
use resources::audio::CLICK;

pub trait ButtonRenderer {
    fn draw(&self, resource_manager: &dyn ResourceManager, highlighted: bool) -> Brick;
}

pub struct Button<R: ButtonRenderer, A> {
    rect: Rect,
    highlighted: Cell<bool>,
    key: Option<Keycode>,
    action: A,
    renderer: R,
}

impl<R: ButtonRenderer, A> Button<R, A> {
    pub fn new(rect: Rect, key: Option<Keycode>, action: A, renderer: R) -> Button<R, A> {
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

    fn on_event(&mut self, event: &Event, resource_manager: &dyn ResourceManager, audio: &Audio) -> EventResult<A> {
        match *event {
            Event::KeyDown(key) if Some(key) == self.key => {
                audio.play(&*resource_manager.chunk(&CLICK)).map_err(err_msg)?;
                Ok(EventReaction::update_and_action(self.action.clone()))
            },
            Event::MouseButtonDown(MouseButton::Left, x, y) if self.get_client_rect().contains_point((x, y)) => {
                audio.play(&*resource_manager.chunk(&CLICK)).map_err(err_msg)?;
                Ok(EventReaction::update_and_action(self.action.clone()))
            },
            Event::MouseMove(x, y) => {
                let to_highlight = self.get_client_rect().contains_point((x, y));
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
