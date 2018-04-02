use ui::context::Context;
use ui::widget::widget::*;
use error::*;

pub struct Modal<A> {
    children: Vec<WidgetPtr<A>>
}

impl<A> Modal<A> {
    pub fn new() -> Self {
        Self { children: Vec::new() }
    }

    pub fn add<W: Widget<A> + 'static>(mut self, child: W) -> Self {
        self.children.push(Box::new(child));
        self
    }

    pub fn push<W: Widget<A> + 'static>(&mut self, child: W) {
        self.children.push(Box::new(child));
    }
}

impl<A> Widget<A> for Modal<A> {
    fn on_event(&self, event: &Event) -> EventReaction<A> {
        for child in self.children.iter().rev() {
            let reaction = child.on_event(event);
            if reaction.is_op() {
                return reaction;
            }
        }
        EventReaction::StopPropagation
    }

    fn draw(&self, context: &Context) -> Result<()> {
        for child in &self.children {
            child.draw(context)?;
        }
        Ok(())
    }
}
