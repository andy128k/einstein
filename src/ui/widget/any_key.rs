use failure::err_msg;
use ui::context::Rect;
use ui::widget::widget::*;
use ui::brick::*;
use resources::manager::ResourceManager;
use resources::audio::CLICK;
use audio::Audio;

pub struct AnyKey<A> {
    action: A
}

impl<A> AnyKey<A> {
    pub fn new(action: A) -> Self {
        Self { action }
    }
}

impl<A> Widget<A> for AnyKey<A> where A: Clone {
    fn is_relative(&self) -> bool { true }

    fn get_rect(&self) -> Rect {
        Rect::default()
    }

    fn on_event(&mut self, event: &Event, resource_manager: &dyn ResourceManager, audio: &Audio) -> EventResult<A> {
        match *event {
            Event::KeyDown(..) | Event::MouseButtonDown(..) => {
                audio.play(&*resource_manager.chunk(&CLICK)).map_err(err_msg)?;
                Ok(EventReaction::action(self.action.clone()))
            },
            _ => Ok(EventReaction::empty()),
        }
    }

    fn draw(&self, _resource_manager: &dyn ResourceManager) -> Brick {
        Brick::new(self.get_rect())
    }
}
