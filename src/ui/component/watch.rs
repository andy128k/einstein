use std::time::{Duration};
use std::rc::Rc;
use std::cell::{Cell};
use cell::RefCell;
use sdl2::pixels::Color;
use ui::context::{Context, Rect, HorizontalAlign};
use ui::widget::common::*;
use ui::widget::brick::*;
use ui::widget::widget::*;
use ui::component::game::GamePrivate;
use resources::manager::ResourceManager;
use error::*;
use util::time::sec_to_str;

pub struct Watch {
    rect: Rect,
    state: Rc<RefCell<GamePrivate>>,
    last_duration: Cell<Option<Duration>>,
}

impl Watch {
    pub fn new(rect: Rect, state: Rc<RefCell<GamePrivate>>) -> Self {
        Self {
            rect,
            state,
            last_duration: Cell::new(None)
        }
    }
}

impl Widget<Nothing> for Watch {
    fn is_relative(&self) -> bool { true }
    fn get_rect(&self) -> Rect { self.rect }

    fn on_event(&mut self, event: &Event) -> EventResult<Nothing> {
        match *event {
            Event::Tick => {
                if Some(self.state.borrow().get_current_duration()) != self.last_duration.get() {
                    Ok(EventReaction::update(self.get_rect()))
                } else {
                    Ok(EventReaction::empty())
                }
            },
            _ => Ok(EventReaction::empty()),
        }
    }

    fn draw(&self, context: &Context, resource_manager: &mut ResourceManager) -> Result<()> {
        let duration = self.state.borrow().get_current_duration();
        self.last_duration.set(Some(duration));

        let s = sec_to_str(duration.as_secs() as u32);

        let brick = Brick::new(self.get_rect())
            .background(BackgroundPattern::Color(Color::RGB(48, 0, 255)))
            .text(Text::new(s).font_size(FontSize::Text).color(Color::RGB(255, 255, 255)).halign(HorizontalAlign::Right));

        brick.draw(context, resource_manager)?;
        Ok(())
    }
}