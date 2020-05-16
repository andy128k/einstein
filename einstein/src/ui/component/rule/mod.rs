use std::rc::Rc;
use std::cell::Cell;
use crate::cell::RefCell;
use sdl2::mouse::MouseButton;
use crate::error::format_err;
use crate::ui::common::Size;
use crate::ui::widget::widget::*;
use crate::ui::widget::common::*;
use crate::ui::brick::*;
use crate::ui::context::Context;
use crate::ui::component::game::{GamePrivate};
use crate::resources::manager::{ResourceManager, Resource};
use crate::resources::audio::WHIZZ;
use crate::resources::thing::{
    get_thing_rect,
    LARGE_THINGS_ATLAS,
    LARGE_THINGS_ATLAS_HIGHLIGHTED,
    EMPTY_TILE,
    EMPTY_TILE_HIGHLIGHTED,
};
use crate::rules::{Rule, Thing};

pub const TILE_WIDTH: u32 = 48;
pub const TILE_HEIGHT: u32 = 48;

const HINT_NEAR_ICON: Resource = resource!("./hint-near.bmp");
const HINT_NEAR_ICON_HIGHLIGHTED: Resource = resource!("./hint-near-h.bmp");

const HINT_SIDE_ICON: Resource = resource!("./hint-side.bmp");
const HINT_SIDE_ICON_HIGHLIGHTED: Resource = resource!("./hint-side-h.bmp");

const HINT_BETWEEN_ICON: Resource = resource!("./betwarr.bmp");
const HINT_BETWEEN_ICON_HIGHLIGHTED: Resource = resource!("./betwarr-h.bmp");

fn draw_thing(thing: Thing, highlighted: bool) -> Brick {
    Brick::new(TILE_WIDTH, TILE_HEIGHT)
        .background(Background::Image(if highlighted { &LARGE_THINGS_ATLAS_HIGHLIGHTED } else { &LARGE_THINGS_ATLAS }, Some(get_thing_rect(thing))))
}

fn draw_rule(rule: &Rule, highlighted: bool) -> Brick {
    let bg = Background::Image(if highlighted { &EMPTY_TILE_HIGHLIGHTED } else { &EMPTY_TILE }, None);
    match *rule {
        Rule::Near(thing1, thing2) => {
            Brick::new(TILE_WIDTH * 3, TILE_HEIGHT)
                .background(bg)
                .add(0, 0, draw_thing(thing1, highlighted))
                .add(TILE_WIDTH, 0, Brick::new(TILE_WIDTH, TILE_HEIGHT).background(Background::Image(if highlighted { &HINT_NEAR_ICON_HIGHLIGHTED } else { &HINT_NEAR_ICON }, None)))
                .add(TILE_WIDTH * 2, 0, draw_thing(thing2, highlighted))
        },
        Rule::Direction(thing1, thing2) => {
            Brick::new(TILE_WIDTH * 3, TILE_HEIGHT)
                .background(bg)
                .add(0, 0, draw_thing(thing1, highlighted))
                .add(TILE_WIDTH * 2, 0, draw_thing(thing2, highlighted))
                .add(TILE_WIDTH, 0, Brick::new(TILE_WIDTH, TILE_HEIGHT).background(Background::Image(if highlighted { &HINT_SIDE_ICON_HIGHLIGHTED } else { &HINT_SIDE_ICON }, None)))
        },
        Rule::Under(thing1, thing2) => {
            Brick::new(TILE_WIDTH, TILE_HEIGHT * 2)
                .background(bg)
                .add(0, 0, draw_thing(thing1, highlighted))
                .add(0, TILE_HEIGHT, draw_thing(thing2, highlighted))
        },
        Rule::Between(thing1, thing2, thing3) => {
            Brick::new(TILE_WIDTH * 3, TILE_HEIGHT)
                .add(0, 0, draw_thing(thing1, highlighted))
                .add(TILE_WIDTH, 0, draw_thing(thing2, highlighted))
                .add(TILE_WIDTH * 2, 0, draw_thing(thing3, highlighted))
                .add((3 * TILE_WIDTH - 70) / 2, 0, Brick::new(70, 15).background(Background::Image(if highlighted { &HINT_BETWEEN_ICON_HIGHLIGHTED } else { &HINT_BETWEEN_ICON }, None)))
        },
        _ => {
            Brick::new(0, 0)
        }
    }
}

pub struct RuleWidget {
    size: Size,
    state: Rc<RefCell<GamePrivate>>,
    index: Option<usize>,
    highlighted: Cell<bool>,
}

impl RuleWidget {
    pub fn new(size: Size, state: Rc<RefCell<GamePrivate>>, index: Option<usize>) -> Self {
        RuleWidget {
            size,
            state,
            index,
            highlighted: Cell::new(false),
        }
    }

    fn get_rule(&self) -> Option<crate::rules::Rule> {
        let index = self.index?;
        let rule = self.state.borrow().rules.get(index)?.clone();
        if self.state.borrow().show_excluded == self.state.borrow().excluded.contains(&index) {
            Some(rule)
        } else {
            None
        }
    }
}

impl Widget<usize> for RuleWidget {
    fn get_size(&self) -> Size { self.size }

    fn on_event(&mut self, event: &Event, context: &dyn Context) -> EventResult<usize> {
        match *event {
            Event::MouseButtonDown(MouseButton::Right, x, y) => {
                if self.get_size().to_rect().contains_point((x, y)) {
                    if self.index.map_or(false, |index| self.state.borrow_mut().toggle_rule(index).is_some()) {
                        context.audio().play(&*context.resource_manager().chunk(&WHIZZ)).map_err(|e| format_err!("{}", e))?;
                        Ok(EventReaction::update())
                    } else {
                        Ok(EventReaction::empty())
                    }
                } else {
                    Ok(EventReaction::empty())
                }
            },
            Event::MouseMove(x, y) => {
                if self.index.is_some() {
                    let inside = self.get_size().to_rect().contains_point((x, y));
                    if inside == self.highlighted.get() {
                        Ok(EventReaction::empty())
                    } else {
                        self.highlighted.set(inside);
                        Ok(EventReaction::update())
                    }
                } else {
                    Ok(EventReaction::empty())
                }
            },
            _ => Ok(EventReaction::empty()),
        }
    }

    fn draw(&self, _resource_manager: &dyn ResourceManager) -> Brick {
        if let Some(rule) = self.get_rule() {
            draw_rule(&rule, self.highlighted.get())
        } else {
            Brick::new(self.size.width, self.size.height)
                .background(Background::Image(if self.highlighted.get() { &EMPTY_TILE_HIGHLIGHTED } else { &EMPTY_TILE }, None))
        }
    }
}
