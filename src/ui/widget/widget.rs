use sdl::video::Surface;
use sdl::event::{Key, Mouse};
use error::*;
pub use algebra::*;

pub enum Event {
    Tick,
    MouseButtonDown(Mouse, i32, i32),
    MouseButtonUp(Mouse, i32, i32),
    MouseMove(i32, i32),
    KeyDown(Key, u16),
}

pub enum EventReaction<A> {
    Action(A),
    Redraw,
    StopPropagation,
    NoOp,
}

impl<A> EventReaction<A> {
    pub fn is_noop(&self) -> bool {
        match self {
            &EventReaction::NoOp => true,
            _ => false,
        }
    }

    pub fn is_op(&self) -> bool {
        !self.is_noop()
    }

    pub fn map_action<B, F: Fn(&A) -> B>(&self, f: F) -> EventReaction<B> {
        match *self {
            EventReaction::Action(ref value) => EventReaction::Action(f(value)),
            EventReaction::Redraw => EventReaction::Redraw,
            EventReaction::StopPropagation => EventReaction::StopPropagation,
            EventReaction::NoOp => EventReaction::NoOp,
        }
    }

    pub fn flatmap_action<B, F: Fn(&A) -> EventReaction<B>>(&self, f: F) -> EventReaction<B> {
        match *self {
            EventReaction::Action(ref value) => f(value),
            EventReaction::Redraw => EventReaction::Redraw,
            EventReaction::StopPropagation => EventReaction::StopPropagation,
            EventReaction::NoOp => EventReaction::NoOp,
        }
    }
}

pub trait Widget<A> {
    fn on_event(&self, _event: &Event) -> EventReaction<A> { EventReaction::NoOp } // TODO Result<EventReaction<A>>
    fn draw(&self, surface: &Surface) -> Result<()>;
}

pub type WidgetPtr<A> = Box<Widget<A>>;

impl<A> Widget<A> for WidgetPtr<A> {
    fn on_event(&self, event: &Event) -> EventReaction<A> {
        (**self).on_event(event)
    }

    fn draw(&self, surface: &Surface) -> Result<()> {
        (**self).draw(surface)
    }
}



pub struct WidgetMapAction<AF, WF, AT> where WF: Widget<AF> {
    wrapped: WF,
    convert: Box<Fn(&AF) -> EventReaction<AT>>,
}

impl<AF, WF, AT> WidgetMapAction<AF, WF, AT> where WF: Widget<AF> {
    pub fn new<F>(wrapped: WF, convert: F) -> Self
        where F: Fn(&AF) -> EventReaction<AT> + 'static
    {
        Self { wrapped, convert: Box::new(convert) }
    }

    pub fn no_action(wrapped: WF) -> Self {
        Self { wrapped, convert: Box::new(|_| EventReaction::Redraw) }
    }
}

impl<AF, WF, AT> Widget<AT> for WidgetMapAction<AF, WF, AT> where WF: Widget<AF> {
    fn on_event(&self, event: &Event) -> EventReaction<AT> {
        let event = self.wrapped.on_event(event);
        event.flatmap_action(&*self.convert)
    }

    fn draw(&self, surface: &Surface) -> Result<()> {
        self.wrapped.draw(surface)
    }
}


impl<'w, A> Widget<A> for Vec<WidgetPtr<A>> {
    fn on_event(&self, event: &Event) -> EventReaction<A> {
        for child in self.iter().rev() {
            let reaction = child.on_event(event);
            if reaction.is_op() {
                return reaction;
            }
        }
        EventReaction::NoOp
    }

    fn draw(&self, surface: &Surface) -> Result<()> {
        for child in self {
            child.draw(surface)?;
        }
        Ok(())
    }
}

impl<W1, W2, A1, A2> Widget<OneOf<A1, A2, Nothing>> for (W1, W2)
    where
        A1: Clone,
        A2: Clone,
        W1: Widget<A1>,
        W2: Widget<A2>,
{
    fn on_event(&self, event: &Event) -> EventReaction<OneOf<A1, A2, Nothing>> {
        let reaction = self.0.on_event(event);
        if reaction.is_op() {
            return reaction.map_action(|a| OneOf::v1(a.clone()));
        }
        let reaction = self.1.on_event(event);
        if reaction.is_op() {
            return reaction.map_action(|a| OneOf::v2(a.clone()));
        }
        EventReaction::NoOp
    }

    fn draw(&self, surface: &Surface) -> Result<()> {
        self.0.draw(surface)?;
        self.1.draw(surface)?;
        Ok(())
    }
}
