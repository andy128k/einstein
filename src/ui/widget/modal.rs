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

fn local_event(event: &Event, rect: Rect) -> Event {
    match *event {
        Event::Tick => Event::Tick,
        Event::MouseButtonDown(mouse, x, y) => Event::MouseButtonDown(mouse, x - rect.left(), y - rect.top()),
        Event::MouseButtonUp(mouse, x, y) => Event::MouseButtonUp(mouse, x - rect.left(), y - rect.top()),
        Event::MouseMove(x, y) => Event::MouseMove(x - rect.left(), y - rect.top()),
        Event::KeyDown(key, ch) => Event::KeyDown(key, ch),
    }
}

impl<A> Widget<A> for Modal<A> {
    fn is_relative(&self) -> bool {
        false
    }

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn on_event(&self, event: &Event) -> EventReaction<A> {
        for child in self.children.iter().rev() {
            if child.is_relative() {
                let event2 = local_event(&local_event(event, self.get_rect()), child.get_rect());
                let reaction = child.on_event(&event2);
                if reaction.is_op() {
                    return reaction;
                }
            } else {
                let reaction = child.on_event(event);
                if reaction.is_op() {
                    return reaction;
                }
            }
        }
        EventReaction::StopPropagation
    }

    fn draw(&self, context: &Context) -> Result<()> {
        for child in &self.children {
            if child.is_relative() {
                let c = context.relative(self.get_rect()).relative(child.get_rect());
                child.draw(&c)?;
            } else {
                child.draw(context)?;
            }
        }
        Ok(())
    }
}
