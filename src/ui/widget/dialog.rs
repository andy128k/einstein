use std::marker::PhantomData;
use std::rc::Rc;
use debug_cell::RefCell;
use ui::context::Context;
use ui::widget::widget::*;
use error::*;

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
    fn on_event(&self, event: &Event) -> EventReaction<A> {
        self.check().unwrap();
        match *self.wrapped.borrow() {
            Some(ref widget) => widget.on_event(event),
            None => EventReaction::NoOp
        }
    }

    fn draw(&self, context: &Context) -> Result<()> {
        self.check()?;
        match *self.wrapped.borrow() {
            Some(ref widget) => widget.draw(context),
            None => Ok(())
        }
    }
}
