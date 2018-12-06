use std::rc::Rc;
use std::cell::{Cell};
use crate::cell::RefCell;
use failure::err_msg;
use sdl2::mouse::MouseButton;
use crate::ui::context::{Rect, Size};
use crate::ui::widget::widget::*;
use crate::ui::widget::common::*;
use crate::ui::brick::*;
use crate::ui::rule::{draw_rule};
use crate::ui::component::game::{GamePrivate};
use crate::resources::manager::ResourceManager;
use crate::resources::thing::EMPTY_TILE;
use crate::resources::audio::WHIZZ;
use crate::audio::Audio;
use crate::error::*;

pub struct VerticalRules {
    rect: Rect,
    state: Rc<RefCell<GamePrivate>>,
    highlighted: Cell<Option<usize>>,
}

const TILE_GAP: u32 =      4;
const TILE_WIDTH: u32 =   48;
const TILE_HEIGHT: u32 =  48;

impl VerticalRules {
    pub fn new(rect: Rect, state: Rc<RefCell<GamePrivate>>) -> Result<Self> {
        Ok(Self {
            rect,
            state,
            highlighted: Cell::new(None),
        })
    }

    fn draw_cell(&self, index: usize) -> Brick {
        if let Some(vertical_rule) = self.state.borrow().vertical_rules.get(index) {
            if self.state.borrow().show_excluded == vertical_rule.is_excluded {
                let rule = &self.state.borrow().rules[vertical_rule.original_index];
                return draw_rule(&rule, self.highlighted.get() == Some(index));
            }
        }

        Brick::new(TILE_WIDTH, TILE_HEIGHT * 2)
            .background(Background::Pattern(&EMPTY_TILE, false))
    }

    fn get_rule_index(&self, x: i32, y: i32) -> Option<usize> {
        let Size { width, height } = self.get_size();
        if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
            return None;
        }
        if (x as u32) % (TILE_WIDTH + TILE_GAP) < TILE_WIDTH {
            let index = (x as u32) / (TILE_WIDTH + TILE_GAP);
            Some(index as usize)
        } else {
            None
        }
    }
}

impl Widget<Nothing> for VerticalRules {
    fn get_size(&self) -> Size {
        Size { width: self.rect.width(), height: self.rect.height() }
    }

    fn on_event(&mut self, event: &Event, resource_manager: &dyn ResourceManager, audio: &Audio) -> EventResult<Nothing> {
        match *event {
            Event::MouseButtonDown(MouseButton::Right, x, y) => {
                match self.get_rule_index(x, y) {
                    Some(no) => {
                        if self.state.borrow_mut().toggle_vertical_rule(no).is_some() {
                            audio.play(&*resource_manager.chunk(&WHIZZ)).map_err(err_msg)?;
                            Ok(EventReaction::update())
                        } else {
                            Ok(EventReaction::empty())
                        }
                    },
                    None => Ok(EventReaction::empty())
                }
            },
            Event::MouseMove(x, y) => {
                let no = self.get_rule_index(x, y);
                if no != self.highlighted.get() {
                    self.highlighted.set(no);
                    Ok(EventReaction::update())
                } else {
                    Ok(EventReaction::empty())
                }
            },
            _ => Ok(EventReaction::empty()),
        }
    }

    fn draw(&self, _resource_manager: &dyn ResourceManager) -> Brick {
        let Size { width, height } = self.get_size();
        let mut brick = Brick::new(width, height);
        let num = (width + TILE_GAP) / (TILE_WIDTH + TILE_GAP);
        for i in 0..num {
            let b = self.draw_cell(i as usize);
            brick.push(i * (TILE_WIDTH + TILE_GAP), 0, b);
        }
        brick
    }
}
