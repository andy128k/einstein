use std::rc::Rc;
use std::cell::Cell;
use crate::cell::RefCell;
use failure::err_msg;
use sdl2::mouse::MouseButton;
use crate::ui::context::Rect;
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

pub struct HorizontalRules {
    rect: Rect,
    state: Rc<RefCell<GamePrivate>>,
    highlighted: Cell<Option<usize>>,
}

const HINTS_COLS: usize =  3;
const HINTS_ROWS: usize =  8;
const TILE_GAP_X: u32 =    4;
const TILE_GAP_Y: u32 =    4;
const TILE_WIDTH: u32 =   48;
const TILE_HEIGHT: u32 =  48;

impl HorizontalRules {
    pub fn new(rect: Rect, state: Rc<RefCell<GamePrivate>>) -> Result<Self> {
        Ok(Self {
            rect,
            state,
            highlighted: Cell::new(None),
        })
    }

    fn draw_cell(&self, index: usize) -> Brick {
        if let Some(horizontal_rule) = self.state.borrow().horizontal_rules.get(index) {
            if self.state.borrow().show_excluded == horizontal_rule.is_excluded {
                let rule = &self.state.borrow().rules[horizontal_rule.original_index];
                return draw_rule(&rule, self.highlighted.get() == Some(index));
            }
        }

        Brick::new(TILE_WIDTH * 3, TILE_HEIGHT)
            .background(Background::Pattern(&EMPTY_TILE, false))
    }

    fn get_rule_index(&self, x: i32, y: i32) -> Option<usize> {
        if !self.get_client_rect().contains_point((x, y)) {
            return None;
        }

        let col = (x as u32) / (TILE_WIDTH*3 + TILE_GAP_X);
        if col * (TILE_WIDTH*3 + TILE_GAP_X) + TILE_WIDTH*3 < (x as u32) {
            return None;
        }

        let row = (y as u32) / (TILE_HEIGHT + TILE_GAP_Y);
        if row * (TILE_HEIGHT + TILE_GAP_Y) + TILE_HEIGHT < (y as u32) {
            return None;
        }
    
        let no = (row as usize) * HINTS_COLS + (col as usize);
        if no >= self.state.borrow().horizontal_rules.len() {
            return None;
        }

        Some(no)
    }
}

impl Widget<Nothing> for HorizontalRules {
    fn is_relative(&self) -> bool { true }
    fn get_rect(&self) -> Rect { self.rect }

    fn on_event(&mut self, event: &Event, resource_manager: &dyn ResourceManager, audio: &Audio) -> EventResult<Nothing> {
        match *event {
            Event::MouseButtonDown(MouseButton::Right, x, y) => {
                match self.get_rule_index(x, y) {
                    Some(no) => {
                        if self.state.borrow_mut().toggle_horizontal_rule(no).is_some() {
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
        let mut brick = Brick::new(self.get_rect().width(), self.get_rect().height());
        // let num_cols = ((self.get_client_rect().width() as i32 + TILE_GAP_X) / (TILE_WIDTH*3 + TILE_GAP_X)) as usize;
        // let num_rows = ((self.get_client_rect().height() as i32 + TILE_GAP_Y) / (TILE_HEIGHT + TILE_GAP_Y)) as usize;
        for i in 0..(HINTS_ROWS * HINTS_COLS) {
            let row = i / HINTS_COLS;
            let col = i % HINTS_COLS;
            brick.push(
                (col as u32) * (TILE_WIDTH*3 + TILE_GAP_X),
                (row as u32) * (TILE_HEIGHT + TILE_GAP_Y),
                self.draw_cell(i)
            );
        }
        brick
    }
}
