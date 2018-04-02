use sdl::event::{Key, Mouse};
use error::*;
use ui::context::{Context, Rect};
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
    fn is_relative(&self) -> bool { false }
    fn get_rect(&self) -> Rect;

    fn get_client_rect(&self) -> Rect {
        let rect = self.get_rect();
        Rect::new(0, 0, rect.width(), rect.height())
    }

    fn on_event(&self, _event: &Event) -> EventReaction<A> { EventReaction::NoOp } // TODO Result<EventReaction<A>>
    fn draw(&self, context: &Context) -> Result<()>;
}

pub type WidgetPtr<A> = Box<Widget<A>>;

impl<A> Widget<A> for WidgetPtr<A> {
    fn is_relative(&self) -> bool {
        (**self).is_relative()
    }

    fn get_rect(&self) -> Rect {
        (**self).get_rect()
    }

    fn on_event(&self, event: &Event) -> EventReaction<A> {
        (**self).on_event(event)
    }

    fn draw(&self, context: &Context) -> Result<()> {
        (**self).draw(context)
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
    fn is_relative(&self) -> bool {
        self.wrapped.is_relative()
    }

    fn get_rect(&self) -> Rect {
        self.wrapped.get_rect()
    }

    fn on_event(&self, event: &Event) -> EventReaction<AT> {
        let event = self.wrapped.on_event(event);
        event.flatmap_action(&*self.convert)
    }

    fn draw(&self, context: &Context) -> Result<()> {
        self.wrapped.draw(context)
    }
}
