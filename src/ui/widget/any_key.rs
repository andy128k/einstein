use failure::err_msg;
use crate::ui::context::Size;
use crate::ui::widget::widget::*;
use crate::ui::brick::*;
use crate::resources::manager::ResourceManager;
use crate::resources::audio::CLICK;
use crate::audio::Audio;

pub struct AnyKey<A> {
    action: A
}

impl<A> AnyKey<A> {
    pub fn new(action: A) -> Self {
        Self { action }
    }
}

impl<A> Widget<A> for AnyKey<A> where A: Clone {
    fn get_size(&self) -> Size {
        Size::EMPTY
    }

    fn on_event(&mut self, event: &Event, resource_manager: &dyn ResourceManager, audio: &dyn Audio) -> EventResult<A> {
        match *event {
            Event::KeyDown(..) | Event::MouseButtonDown(..) => {
                audio.play(&*resource_manager.chunk(&CLICK)).map_err(err_msg)?;
                Ok(EventReaction::action(self.action.clone()))
            },
            _ => Ok(EventReaction::empty()),
        }
    }

    fn draw(&self, _resource_manager: &dyn ResourceManager) -> Brick {
        Brick::new(0, 0)
    }
}
