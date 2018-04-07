use error::*;
use ui::context::{Context, Rect};
use ui::widget::widget::*;

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

    fn on_event(&mut self, event: &Event) -> EventResult<A> {
        match *event {
            Event::KeyDown(..) | Event::MouseButtonDown(..) => {
                // sound->play(L"click.wav");
                Ok(EventReaction::action(self.action.clone()))
            },
            _ => Ok(EventReaction::empty()),
        }
    }

    fn draw(&self, _context: &Context) -> Result<()> {
        Ok(())
    }
}
