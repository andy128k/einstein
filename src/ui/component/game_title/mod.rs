use std::time::{Duration};
use std::rc::Rc;
use std::cell::{Cell};
use cell::RefCell;
use sdl2::pixels::Color;
use ui::context::{Rect, HorizontalAlign};
use ui::widget::common::*;
use ui::widget::brick::*;
use ui::widget::widget::*;
use ui::component::game::GamePrivate;
use resources::manager::ResourceManager;
use util::time::sec_to_str;

const GAME_TITLE: &[u8] = include_bytes!("./title.bmp");

pub struct GameTitle {
    rect: Rect,
    title: String,
    state: Rc<RefCell<GamePrivate>>,
    last_duration: Cell<Option<Duration>>,
}

impl GameTitle {
    pub fn new(rect: Rect, title: &str, state: Rc<RefCell<GamePrivate>>) -> Self {
        GameTitle {
            rect,
            title: title.to_owned(),
            state,
            last_duration: Cell::new(None)
        }
    }
}

impl Widget<Nothing> for GameTitle {
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

    fn draw(&self, _resource_manager: &mut ResourceManager) -> Brick {
        let duration = self.state.borrow().get_current_duration();
        self.last_duration.set(Some(duration));
        let s = sec_to_str(duration.as_secs() as u32);

        let watch_rect = Rect::new(
            (self.get_rect().width() - 7 - 100) as i32,
            7,
            100,
            self.get_rect().height() - (2 * 7)
        );

        Brick::new(self.get_rect())
            .background(BackgroundPattern::Custom("GAME_TITLE", GAME_TITLE, false))
            .text(Text::new(&self.title).font_size(FontSize::TITLE).color(Color::RGB(255, 255, 0)).shadow().halign(HorizontalAlign::Center))
            .add(Brick::new(watch_rect)
                .background(BackgroundPattern::Color(Color::RGB(48, 0, 255)))
                .text(Text::new(s).font_size(FontSize::TEXT).color(Color::RGB(255, 255, 255)).halign(HorizontalAlign::Right))
            )
    }
}
