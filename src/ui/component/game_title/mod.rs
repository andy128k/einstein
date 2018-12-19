use std::time::{Duration};
use std::rc::Rc;
use std::cell::{Cell};
use crate::cell::RefCell;
use sdl2::pixels::Color;
use crate::ui::common::{Size, HorizontalAlign};
use crate::ui::widget::common::*;
use crate::ui::brick::*;
use crate::ui::widget::widget::*;
use crate::ui::component::game::GamePrivate;
use crate::resources::manager::{ResourceManager, Resource};
use crate::audio::Audio;
use crate::util::time::sec_to_str;

const GAME_TITLE: Resource = resource!("./title.bmp");
const PADDING: u32 = 7;
const WATCH_WIDTH: u32 = 100;

pub struct GameTitle {
    title: String,
    state: Rc<RefCell<GamePrivate>>,
    last_duration: Cell<Option<Duration>>,
}

impl GameTitle {
    pub fn new(title: &str, state: Rc<RefCell<GamePrivate>>) -> Self {
        GameTitle {
            title: title.to_owned(),
            state,
            last_duration: Cell::new(None)
        }
    }
}

impl Widget<Nothing> for GameTitle {
    fn get_size(&self) -> Size {
        Size { width: 783, height: 47 }
    }

    fn on_event(&mut self, event: &Event, _resource_manager: &dyn ResourceManager, _audio: &dyn Audio) -> EventResult<Nothing> {
        match *event {
            Event::Tick => {
                if Some(self.state.borrow().get_current_duration()) != self.last_duration.get() {
                    Ok(EventReaction::update())
                } else {
                    Ok(EventReaction::empty())
                }
            },
            _ => Ok(EventReaction::empty()),
        }
    }

    fn draw(&self, _resource_manager: &dyn ResourceManager) -> Brick {
        let duration = self.state.borrow().get_current_duration();
        self.last_duration.set(Some(duration));
        let s = sec_to_str(duration.as_secs() as u32);

        Brick::new(self.get_size().width, self.get_size().height)
            .background(Background::Pattern(&GAME_TITLE, false))
            .text(Text::new(&self.title).font_size(FontSize::TITLE).color(Color::RGB(255, 255, 0)).shadow().halign(HorizontalAlign::Center))
            .add(
                self.get_size().width - PADDING - WATCH_WIDTH,
                PADDING,
                Brick::new(WATCH_WIDTH, self.get_size().height - (2 * PADDING))
                    .background(Background::Color(Color::RGB(48, 0, 255)))
                    .text(Text::new(s).font_size(FontSize::TEXT).color(Color::RGB(255, 255, 255)).halign(HorizontalAlign::Right))
            )
    }
}
