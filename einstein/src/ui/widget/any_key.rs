use crate::error::format_err;
use crate::ui::common::Size;
use crate::ui::widget::widget::*;
use crate::ui::brick::*;
use crate::ui::context::Context;
use crate::resources::manager::ResourceManager;
use crate::resources::audio::CLICK;

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

    fn on_event(&mut self, event: &Event, context: &dyn Context) -> EventResult<A> {
        match *event {
            Event::KeyDown(..) | Event::MouseButtonDown(..) => {
                context
                    .audio()
                    .play(&*context.resource_manager().chunk(&CLICK))
                    .map_err(|e| format_err!("{}", e))?;
                Ok(EventReaction::action(self.action.clone()))
            },
            _ => Ok(EventReaction::empty()),
        }
    }

    fn draw(&self, _resource_manager: &dyn ResourceManager) -> Brick {
        Brick::new(0, 0)
    }
}
