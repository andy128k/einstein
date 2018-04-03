use ui::context::{Context, Rect};
use ui::widget::widget::*;
use error::*;

pub struct Modal<A> {
    rect: Rect,
    children: Vec<WidgetPtr<A>>
}

impl<A> Modal<A> {
    pub fn new(rect: Rect) -> Self {
        Self { rect, children: Vec::new() }
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
    fn is_relative(&self) -> bool {
        true
    }

    fn get_rect(&self) -> Rect { self.rect }

    fn on_event(&self, event: &Event) -> EventReaction<A> {
        for child in self.children.iter().rev() {
            let event2 = event.relative(child.get_rect());
            let reaction = child.on_event(&event2);
            if reaction.is_op() {
                return reaction;
            }
        }
        EventReaction::StopPropagation
    }

    fn draw(&self, context: &Context) -> Result<()> {
        for child in &self.children {
            child.draw(&context.relative(child.get_rect()))?;
        }
        Ok(())
    }
}
