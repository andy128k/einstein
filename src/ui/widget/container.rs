use crate::ui::context::Rect;
use crate::ui::widget::widget::*;
use crate::ui::widget::common::*;
use crate::ui::brick::*;
use crate::resources::manager::ResourceManager;
use crate::audio::Audio;

pub struct Container<A> {
    rect: Rect,
    background: Background,
    children: Vec<WidgetPtr<A>>,
    modal: bool,
}

impl<A> Container<A> {
    pub fn container(rect: Rect, background: Background) -> Self {
        Container { rect, background, children: Vec::new(), modal: false }
    }

    pub fn modal(rect: Rect, background: Background) -> Self {
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

    fn on_event(&mut self, event: &Event, resource_manager: &dyn ResourceManager, audio: &Audio) -> EventResult<A> {
        let mut reaction = EventReaction::empty();
        for child in self.children.iter_mut().rev() {
            let event2 = event.relative(child.get_rect());
            let child_reaction = child.on_event(&event2, resource_manager, audio)?;
            // TODO -relative
            reaction.add(&child_reaction);
            if reaction.terminated {
                break;
            }
        }
        reaction.terminated = self.modal;
        Ok(reaction)
    }

    fn draw(&self, resource_manager: &dyn ResourceManager) -> Brick {
        let mut brick = Brick::new(self.get_rect().width(), self.get_rect().height())
            .background(self.background);

        match self.background {
            Background::Pattern(..) => {},
            _ => {
                brick = brick.border(Border::Raised);
            },
        }

        for child in &self.children {
            let cb = child.draw(resource_manager);
            brick.push(child.get_rect().left() as u32, child.get_rect().top() as u32, cb);
        }

        brick
    }
}
