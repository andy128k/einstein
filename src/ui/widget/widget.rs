use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use error::*;
use ui::context::Rect;
use ui::widget::brick::Brick;
use resources::manager::ResourceManager;
pub use algebra::*;

pub enum Event {
    Tick,
    MouseButtonDown(MouseButton, i32, i32),
    MouseButtonUp(MouseButton, i32, i32),
    MouseMove(i32, i32),
    KeyDown(Keycode),
    TextInput(String),
}

impl Event {
    pub fn relative(&self, rect: Rect) -> Self {
        match *self {
            Event::Tick => Event::Tick,
            Event::MouseButtonDown(mouse, x, y) => Event::MouseButtonDown(mouse, x - rect.left(), y - rect.top()),
            Event::MouseButtonUp(mouse, x, y) => Event::MouseButtonUp(mouse, x - rect.left(), y - rect.top()),
            Event::MouseMove(x, y) => Event::MouseMove(x - rect.left(), y - rect.top()),
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
            terminated: false
        }
    }

    pub fn update() -> Self {
        Self {
            update: true,
            action: None,
            terminated: false
        }
    }

    pub fn update_and_action(action: A) -> Self {
        Self {
            update: true,
            action: Some(action),
            terminated: false
        }
    }

    pub fn action(action: A) -> Self {
        Self {
            update: false,
            action: Some(action),
            terminated: false
        }
    }
}

impl<A> EventReaction<A> where A: Clone {
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
    fn is_relative(&self) -> bool;
    fn get_rect(&self) -> Rect;

    fn get_client_rect(&self) -> Rect {
        let rect = self.get_rect();
        Rect::new(0, 0, rect.width(), rect.height())
    }

    fn on_event(&mut self, _event: &Event) -> EventResult<A> {
        Ok(EventReaction::empty())
    }
    fn draw(&self, resource_manager: &dyn ResourceManager) -> Brick;
}

pub type WidgetPtr<A> = Box<Widget<A>>;

impl<A> Widget<A> for WidgetPtr<A> {
    fn is_relative(&self) -> bool {
        (**self).is_relative()
    }

    fn get_rect(&self) -> Rect {
        (**self).get_rect()
    }

    fn on_event(&mut self, event: &Event) -> EventResult<A> {
        (**self).on_event(event)
    }

    fn draw(&self, resource_manager: &dyn ResourceManager) -> Brick {
        (**self).draw(resource_manager)
    }
}



pub struct WidgetMapAction<AF, WF, AT> where WF: Widget<AF> {
    wrapped: WF,
    convert: Box<Fn(&AF) -> Result<EventReaction<AT>>>,
}

impl<AF, WF, AT> WidgetMapAction<AF, WF, AT> where WF: Widget<AF> {
    pub fn new<F>(wrapped: WF, convert: F) -> Self
        where F: Fn(&AF) -> Result<EventReaction<AT>> + 'static
    {
        Self { wrapped, convert: Box::new(convert) }
    }

    pub fn no_action(wrapped: WF) -> Self {
        Self { wrapped, convert: Box::new(|_| Ok(EventReaction::empty())) }
    }
}

impl<AF, WF, AT> Widget<AT> for WidgetMapAction<AF, WF, AT> where WF: Widget<AF> {
    fn is_relative(&self) -> bool {
        self.wrapped.is_relative()
    }

    fn get_rect(&self) -> Rect {
        self.wrapped.get_rect()
    }

    fn on_event(&mut self, event: &Event) -> EventResult<AT> {
        let reaction = self.wrapped.on_event(event)?;
        if let Some(ref action) = reaction.action {
            let mut reaction2 = (self.convert)(action)?;
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
