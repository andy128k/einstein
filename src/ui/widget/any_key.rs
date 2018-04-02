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
    fn get_rect(&self) -> Rect {
        Rect::new(-10000, -10000, 1, 1)
    }

    fn on_event(&self, event: &Event) -> EventReaction<A> {
        match *event {
            Event::KeyDown(..) | Event::MouseButtonDown(..) => {
                // sound->play(L"click.wav");
                EventReaction::Action(self.action.clone())
            },
            _ => EventReaction::NoOp,
        }
    }

    fn draw(&self, _context: &Context) -> Result<()> {
        Ok(())
    }
}
