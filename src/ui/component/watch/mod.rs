use std::time::{Duration};
use std::rc::Rc;
use std::cell::{Cell};
use debug_cell::RefCell;
use sdl::video::Surface;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use ui::context::Context;
use ui::widget::widget::*;
use ui::utils::{draw_text, HorizontalAlign, VerticalAlign, rect2_to_rect};
use resources::fonts::text_font;
use error::*;
use util::time::sec_to_str;
use ui::component::game::GamePrivate;

const APP_WIDTH: u32 =          800;
const TITLE_RIGHT: u32 =          9;
const TITLE_TOP: u32 =            8;
const TITLE_PADDING_RIGHT: u32 =  7;
const TITLE_PADDING_TOP: u32 =    7;
const WATCH_WIDTH: u32 =        100;
const WATCH_HEIGHT: u32 =        34;

pub struct Watch {
    state: Rc<RefCell<GamePrivate>>,
    last_duration: Cell<Option<Duration>>,
}

impl Watch {
    pub fn new(state: Rc<RefCell<GamePrivate>>) -> Self {
        Self {
            state,
            last_duration: Cell::new(None)
        }
    }

    fn get_rect(&self) -> Rect {
        Rect::new(
            (APP_WIDTH - TITLE_RIGHT - TITLE_PADDING_RIGHT - WATCH_WIDTH) as i32,
            (TITLE_TOP + TITLE_PADDING_TOP) as i32,
            WATCH_WIDTH,
            WATCH_HEIGHT
        )
    }
}

impl Widget<Nothing> for Watch {
    fn on_event(&self, event: &Event) -> EventReaction<Nothing> {
        match *event {
            Event::Tick => {
                if Some(self.state.borrow().get_current_duration()) != self.last_duration.get() {
                    EventReaction::Redraw
                } else {
                    EventReaction::NoOp
                }
            },
            _ => EventReaction::NoOp,
        }
    }

    fn draw(&self, context: &Context) -> Result<()> {
        let duration = self.state.borrow().get_current_duration();
        self.last_duration.set(Some(duration));

        let s = sec_to_str(duration.as_secs() as u32);

        let c = context.relative(self.get_rect());
        c.fill(Color::RGB(48, 0, 255));
        c.text(&s, text_font()?, Color::RGB(255, 255, 255), true, HorizontalAlign::Right, VerticalAlign::Middle)?;
        Ok(())
    }
}
