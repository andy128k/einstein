pub use crate::algebra::*;
use crate::error::*;
use crate::resources::manager::ResourceManager;
use crate::ui::brick::Brick;
use crate::ui::common::Size;
use crate::ui::context::Context;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

pub enum Event {
    Tick,
    MouseButtonDown(MouseButton, i32, i32),
    MouseButtonUp(MouseButton, i32, i32),
    MouseMove(i32, i32),
    KeyDown(Keycode),
    TextInput(String),
}

impl Event {
    pub fn relative(&self, left: i32, top: i32) -> Self {
        match *self {
            Event::Tick => Event::Tick,
            Event::MouseButtonDown(mouse, x, y) => Event::MouseButtonDown(mouse, x - left, y - top),
            Event::MouseButtonUp(mouse, x, y) => Event::MouseButtonUp(mouse, x - left, y - top),
            Event::MouseMove(x, y) => Event::MouseMove(x - left, y - top),
            Event::KeyDown(key) => Event::KeyDown(key),
            Event::TextInput(ref text) => Event::TextInput(text.clone()),
        }
    }
}

pub struct EventReaction<A> {
    pub update: bool,
    pub action: Option<A>,
    pub terminated: bool,
}

impl<A> EventReaction<A> {
    pub fn empty() -> Self {
        Self {
            update: false,
            action: None,
            terminated: false,
        }
    }

    pub fn update() -> Self {
        Self {
            update: true,
            action: None,
            terminated: false,
        }
    }

    pub fn update_and_action(action: A) -> Self {
        Self {
            update: true,
            action: Some(action),
            terminated: false,
        }
    }

    pub fn action(action: A) -> Self {
        Self {
            update: false,
            action: Some(action),
            terminated: false,
        }
    }
}

impl<A> EventReaction<A>
where
    A: Clone,
{
    pub fn add(&mut self, reaction2: &EventReaction<A>) {
        if !self.terminated {
            self.terminated |= reaction2.terminated;
            self.update |= reaction2.update;
            self.action = match (&self.action, &reaction2.action) {
                (&None, &None) => None,
                (&Some(ref a), &None) => Some((*a).clone()),
                (_, &Some(ref a)) => Some((*a).clone()),
            };
        }
    }
}

pub type EventResult<A> = Result<EventReaction<A>>;

pub trait Widget<A> {
    fn get_size(&self) -> Size;

    fn on_event(&mut self, _event: &Event, _context: &dyn Context) -> EventResult<A> {
        Ok(EventReaction::empty())
    }

    fn draw(&self, resource_manager: &dyn ResourceManager) -> Brick;
}

pub type WidgetPtr<A> = Box<dyn Widget<A>>;

impl<A> Widget<A> for WidgetPtr<A> {
    fn get_size(&self) -> Size {
        (**self).get_size()
    }

    fn on_event(&mut self, event: &Event, context: &dyn Context) -> EventResult<A> {
        (**self).on_event(event, context)
    }

    fn draw(&self, resource_manager: &dyn ResourceManager) -> Brick {
        (**self).draw(resource_manager)
    }
}

pub struct WidgetMapAction<AF, WF, AT>
where
    WF: Widget<AF>,
{
    wrapped: WF,
    convert: Box<dyn Fn(&AF, &dyn Context) -> Result<EventReaction<AT>>>,
}

impl<AF, WF, AT> Widget<AT> for WidgetMapAction<AF, WF, AT>
where
    WF: Widget<AF>,
{
    fn get_size(&self) -> Size {
        self.wrapped.get_size()
    }

    fn on_event(&mut self, event: &Event, context: &dyn Context) -> EventResult<AT> {
        let reaction = self.wrapped.on_event(event, context)?;
        if let Some(ref action) = reaction.action {
            let mut reaction2 = (self.convert)(action, context)?;
            reaction2.update |= reaction.update;
            reaction2.terminated |= reaction.terminated;
            Ok(reaction2)
        } else {
            Ok(EventReaction {
                update: reaction.update,
                action: None,
                terminated: reaction.terminated,
            })
        }
    }

    fn draw(&self, resource_manager: &dyn ResourceManager) -> Brick {
        self.wrapped.draw(resource_manager)
    }
}

pub trait WidgetMapActionExt<A, WT, AT, F>
where
    WT: Widget<AT>,
    F: Fn(&A) -> AT + 'static,
{
    fn map_action(self, convert: F) -> WT;
}

impl<A, AnyWidget, AT, F> WidgetMapActionExt<A, WidgetMapAction<A, Self, AT>, AT, F> for AnyWidget
where
    AnyWidget: Widget<A>,
    F: Fn(&A) -> AT + 'static,
{
    fn map_action(self, convert: F) -> WidgetMapAction<A, Self, AT> {
        WidgetMapAction {
            wrapped: self,
            convert: Box::new(move |a, _| Ok(EventReaction::action(convert(a)))),
        }
    }
}

pub trait WidgetFlatMapActionExt<A, WT, AT, F>
where
    WT: Widget<AT>,
    F: Fn(&A, &dyn Context) -> Result<EventReaction<AT>> + 'static,
{
    fn flat_map_action(self, convert: F) -> WT;
}

impl<A, AnyWidget, AT, F> WidgetFlatMapActionExt<A, WidgetMapAction<A, Self, AT>, AT, F>
    for AnyWidget
where
    AnyWidget: Widget<A>,
    F: Fn(&A, &dyn Context) -> Result<EventReaction<AT>> + 'static,
{
    fn flat_map_action(self, convert: F) -> WidgetMapAction<A, Self, AT> {
        WidgetMapAction {
            wrapped: self,
            convert: Box::new(convert),
        }
    }
}

pub trait WidgetNoActionExt<W, A>
where
    W: Widget<A>,
{
    fn no_action(self) -> W;
}

impl<A, AnyWidget, AT> WidgetNoActionExt<WidgetMapAction<A, Self, AT>, AT> for AnyWidget
where
    AnyWidget: Widget<A>,
{
    fn no_action(self) -> WidgetMapAction<A, Self, AT> {
        WidgetMapAction {
            wrapped: self,
            convert: Box::new(|_, _| Ok(EventReaction::empty())),
        }
    }
}
