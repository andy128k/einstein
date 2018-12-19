use crate::ui::common::Size;
use crate::ui::widget::widget::*;
use crate::ui::widget::common::*;
use crate::ui::brick::*;
use crate::resources::manager::ResourceManager;
use crate::audio::Audio;

struct Child<A> {
    left: u32,
    top: u32,
    widget: WidgetPtr<A>,
}

pub struct Container<A> {
    size: Size,
    background: Option<Background>,
    border: Option<Border>,
    children: Vec<Child<A>>,
    modal: bool,
}

impl<A> Container<A> {
    pub fn container(size: Size, background: impl Into<Option<Background>>, border: impl Into<Option<Border>>) -> Self {
        Container { size, background: background.into(), border: border.into(), children: Vec::new(), modal: false }
    }

    pub fn modal(size: Size, background: impl Into<Option<Background>>) -> Self {
        Container { size, background: background.into(), border: None, children: Vec::new(), modal: true }
    }

    pub fn screen_modal(background: impl Into<Option<Background>>) -> Self {
        Container { size: Size::new(800, 600), background: background.into(), border: None, children: Vec::new(), modal: true }
    }

    pub fn add<W: Widget<A> + 'static>(mut self, left: u32, top: u32, child: W) -> Self {
        self.children.push(Child {
            left,
            top,
            widget: Box::new(child),
        });
        self
    }

    pub fn push<W: Widget<A> + 'static>(&mut self, left: u32, top: u32, child: W) {
        self.children.push(Child {
            left,
            top,
            widget: Box::new(child),
        });
    }
}

impl<A> Widget<A> for Container<A> where A: Clone {
    fn get_size(&self) -> Size { self.size }

    fn on_event(&mut self, event: &Event, resource_manager: &dyn ResourceManager, audio: &dyn Audio) -> EventResult<A> {
        let mut reaction = EventReaction::empty();
        for child in self.children.iter_mut().rev() {
            let event2 = event.relative(child.left as i32, child.top as i32);
            let child_reaction = child.widget.on_event(&event2, resource_manager, audio)?;
            reaction.add(&child_reaction);
            if reaction.terminated {
                break;
            }
        }
        reaction.terminated = self.modal;
        Ok(reaction)
    }

    fn draw(&self, resource_manager: &dyn ResourceManager) -> Brick {
        let mut brick = Brick::new(self.get_size().width, self.get_size().height);
        if let Some(bg) = self.background {
            brick = brick.background(bg);
        }
        if let Some(border) = self.border {
            brick = brick.border(border);
        }

        for child in &self.children {
            let cb = child.widget.draw(resource_manager);
            brick.push(child.left, child.top, cb);
        }

        brick
    }
}
