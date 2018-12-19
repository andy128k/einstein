use std::rc::Rc;
use std::cell::{Cell};
use crate::cell::RefCell;
use sdl2::mouse::MouseButton;
use crate::rules::{Thing};
use crate::ui::common::{Rect, Size};
use crate::ui::widget::widget::*;
use crate::ui::widget::common::*;
use crate::ui::brick::*;
use crate::ui::component::game::GamePrivate;
use crate::resources::manager::ResourceManager;
use crate::resources::thing::{get_thing_rect, get_small_thing_rect, LARGE_THINGS_ATLAS, SMALL_THINGS_ATLAS, EMPTY_TILE};
use crate::resources::audio::LASER;
use crate::audio::Audio;

const PUZZLE_SIZE: u8 = 6;

const FIELD_TILE_WIDTH: u32 =  48;
const FIELD_TILE_HEIGHT: u32 = 48;

fn local_choice_cell_rect(value: u8) -> Rect {
    let x: i32 = ((value % 3) as i32) * ((FIELD_TILE_WIDTH / 3) as i32);
    let y: i32 = ((value / 3) as i32) * ((FIELD_TILE_HEIGHT / 3) as i32) + ((FIELD_TILE_HEIGHT / 6) as i32);
    Rect::new(x, y, (FIELD_TILE_WIDTH / 3) as u32, (FIELD_TILE_HEIGHT / 3) as u32)
}

fn local_find_choice(x: i32, y: i32) -> Option<u8> {
    for i in 0..PUZZLE_SIZE {
        if local_choice_cell_rect(i).contains_point((x, y)) {
            return Some(i);
        }
    }
    None
}

#[derive(Clone)]
pub enum PuzzleAction {
    Victory,
    Failure,
}

pub struct PuzzleCell {
    state: Rc<RefCell<GamePrivate>>,
    row: u8,
    col: u8,
    highlighted: Cell<Option<Option<u8>>>,
}

impl PuzzleCell {
    pub fn new(state: &Rc<RefCell<GamePrivate>>, row: u8, col: u8) -> Self {
        Self {
            state: state.clone(),
            row,
            col,
            highlighted: Cell::new(None),
        }
    }

    fn on_mouse_button_down(&self, button: MouseButton, x: i32, y: i32, resource_manager: &dyn ResourceManager, audio: &dyn Audio) -> EventReaction<PuzzleAction> {
        if self.state.borrow().possibilities.is_defined(self.col, self.row) {
            return EventReaction::empty();
        }

        let Size { width, height } = self.get_size();
        if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
            return EventReaction::empty();
        }

        let value = match local_find_choice(x, y) {
            Some(v) => v,
            None => return EventReaction::empty()
        };
        let thing = Thing { row: self.row, value };

        match button {
            MouseButton::Left => {
                if self.state.borrow().possibilities.is_possible(self.col, thing) {
                    let p = self.state.borrow().possibilities.set(self.col, self.row, thing.value);
                    self.state.borrow_mut().possibilities = p;
                    audio.play(&*resource_manager.chunk(&LASER)).unwrap();
                }
            },
            MouseButton::Right => {
                if self.state.borrow().possibilities.is_possible(self.col, thing) {
                    let p = self.state.borrow().possibilities.exclude(self.col, self.row, thing.value);
                    self.state.borrow_mut().possibilities = p;
                    audio.play(&*resource_manager.chunk(&LASER)).unwrap();
                }
            },
            _ => {}
        }

        if !self.state.borrow().is_valid() {
            EventReaction::action(PuzzleAction::Failure)
        } else if self.state.borrow().possibilities.is_solved() {
            EventReaction::action(PuzzleAction::Victory)
        } else {
            EventReaction::update()
        }
    }

    fn on_mouse_move(&self, x: i32, y: i32) -> bool {
        if !self.get_size().to_rect().contains_point((x,y)) && self.highlighted.get().is_none() {
            return false;
        }

        if self.state.borrow().possibilities.is_defined(self.col, self.row) {
            if self.get_size().to_rect().contains_point((x,y)) {
                self.highlighted.set(Some(None));
            } else {
                self.highlighted.set(None);
            }
        } else {
            match local_find_choice(x, y) {
                Some(p) => self.highlighted.set(Some(Some(p))),
                None => self.highlighted.set(None)
            }
        }

        true
    }
}

impl Widget<PuzzleAction> for PuzzleCell {
    fn get_size(&self) -> Size {
        Size { width: FIELD_TILE_WIDTH as u32, height: FIELD_TILE_HEIGHT as u32 }
    }

    fn on_event(&mut self, event: &Event, resource_manager: &dyn ResourceManager, audio: &dyn Audio) -> EventResult<PuzzleAction> {
        match *event {
            Event::MouseButtonDown(button, x, y) => Ok(self.on_mouse_button_down(button, x, y, resource_manager, audio)),
            Event::MouseMove(x, y) => {
                if self.on_mouse_move(x, y) {
                    Ok(EventReaction::update())
                } else {
                    Ok(EventReaction::empty())
                }
            },
            _ => Ok(EventReaction::empty()),
        }
    }

    fn draw(&self, _resource_manager: &dyn ResourceManager) -> Brick {
        let row = self.row;
        let col = self.col;

        let mut brick = Brick::new(FIELD_TILE_WIDTH, FIELD_TILE_HEIGHT);

        if let Some(value) = self.state.borrow().possibilities.get_defined(col, row) {
            let thing = Thing { row, value };
            let highlight = self.highlighted.get() == Some(None);

            let rect = get_thing_rect(thing);
            brick = brick.background(Background::Sprite(&LARGE_THINGS_ATLAS, highlight, rect));
        } else {
            brick = brick.background(Background::Pattern(&EMPTY_TILE, false));

            for i in 0..PUZZLE_SIZE {
                let choice_rect = local_choice_cell_rect(i);
                let thing = Thing { row, value: i };
                if self.state.borrow().possibilities.is_possible(col as u8, thing) {
                    let highlight = self.highlighted.get() == Some(Some(i));

                    let rect = get_small_thing_rect(thing);
                    brick.push(
                        choice_rect.left() as u32,
                        choice_rect.top() as u32,
                        Brick::new(choice_rect.width(), choice_rect.height())
                            .background(Background::Sprite(&SMALL_THINGS_ATLAS, highlight, rect))
                    );
                }
            }
        }
        brick
    }
}
