use std::marker::PhantomData;
use std::rc::Rc;
use crate::cell::RefCell;
use crate::ui::context::Size;
use crate::ui::widget::widget::*;
use crate::ui::brick::*;
use crate::resources::manager::ResourceManager;
use crate::audio::Audio;
use crate::error::*;

pub struct ConditionalWidget<A, W, I> where W: Widget<A> {
    wrapped: RefCell<Option<W>>,
    factory: Box<Fn(&I) -> Result<W>>,
    condition: Rc<RefCell<Option<I>>>,
    phantom: PhantomData<A>,
}

impl<A, W, I> ConditionalWidget<A, W, I> where W: Widget<A> {
    pub fn new<F>(condition: Rc<RefCell<Option<I>>>, f: F) -> Self
        where F: Fn(&I) -> Result<W> + 'static
    {
        Self {
            wrapped: RefCell::new(None),
            factory: Box::new(f),
            condition,
            phantom: PhantomData
        }
    }

    fn check(&self) -> Result<()> {
        if let Some(ref init) = *self.condition.borrow() {
            let not_created = self.wrapped.borrow().is_none();
            if not_created {
                let widget = (*self.factory)(init)?;
                *self.wrapped.borrow_mut() = Some(widget);
            }
        } else {
            let created = self.wrapped.borrow().is_some();
            if created {
                *self.wrapped.borrow_mut() = None;
            }
        }
        Ok(())
    }
}

impl<A, W, I> Widget<A> for ConditionalWidget<A, W, I> where W: Widget<A> {
    fn get_size(&self) -> Size {
        self.check().unwrap();
        match *self.wrapped.borrow() {
            Some(ref widget) => widget.get_size(),
            None => Size::EMPTY,
        }
    }

    fn on_event(&mut self, event: &Event, resource_manager: &dyn ResourceManager, audio: &Audio) -> EventResult<A> {
        self.check()?;
        match *self.wrapped.borrow_mut() {
            Some(ref mut widget) => widget.on_event(event, resource_manager, audio),
            None => Ok(EventReaction::empty())
        }
    }

    fn draw(&self, resource_manager: &dyn ResourceManager) -> Brick {
        self.check().unwrap();
        match *self.wrapped.borrow() {
            Some(ref widget) => widget.draw(resource_manager),
            None => Brick::new(0, 0)
        }
    }
}
