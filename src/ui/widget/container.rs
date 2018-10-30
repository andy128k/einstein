use ui::context::Rect;
use ui::widget::widget::*;
use ui::widget::common::*;
use ui::widget::brick::*;
use resources::manager::ResourceManager;

pub struct Container<A> {
    rect: Rect,
    background: BackgroundPattern,
    children: Vec<WidgetPtr<A>>,
    modal: bool,
}

impl<A> Container<A> {
    pub fn container(rect: Rect, background: BackgroundPattern) -> Self {
        Container { rect, background, children: Vec::new(), modal: false }
    }

    pub fn modal(rect: Rect, background: BackgroundPattern) -> Self {
        Container { rect, background, children: Vec::new(), modal: true }
    }

    pub fn add<W: Widget<A> + 'static>(mut self, child: W) -> Self {
        self.children.push(Box::new(child));
        self
    }

    pub fn push<W: Widget<A> + 'static>(&mut self, child: W) {
        self.children.push(Box::new(child));
    }
}

impl<A> Widget<A> for Container<A> where A: Clone {
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
        reaction.terminated = self.modal;
        Ok(reaction)
    }

    fn draw(&self, resource_manager: &mut ResourceManager) -> Brick {
        let mut brick = Brick::new(self.get_rect())
            .background(self.background);

        match self.background {
            BackgroundPattern::Custom(..) => {},
            _ => {
                brick = brick.border(Border::Raised);
            },
        }

        for child in &self.children {
            let cb = child.draw(resource_manager);
            brick.push(cb);
        }

        brick
    }
}
