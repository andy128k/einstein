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

impl<A> Widget<A> for Modal<A> where A: Clone {
    fn is_relative(&self) -> bool {
        true
    }

    fn get_rect(&self) -> Rect { self.rect }

    fn on_event(&mut self, event: &Event) -> EventResult<A> {
        let mut reaction = EventReaction::empty();
        for child in self.children.iter_mut().rev() {
            let event2 = event.relative(child.get_rect());
            let child_reaction = child.on_event(&event2)?;
            // TODO -relative
            reaction.add(&child_reaction);
            if reaction.terminated {
                break;
            }
        }
        reaction.terminated = true;
        Ok(reaction)
    }

    fn draw(&self, context: &Context) -> Result<()> {
        for child in &self.children {
            child.draw(&context.relative(child.get_rect()))?;
        }
        Ok(())
    }
}
